use actix_cors::Cors;
use actix_service::Service;
#[allow(unused_imports)]
use actix_web::{get, post, put, web, App, http, http::{HeaderName, HeaderValue, Cookie}, HttpResponse, HttpRequest, HttpServer, Responder, Result, Error, client::Client, middleware::Logger, cookie::SameSite};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_files as fs;
#[allow(unused_imports)]
use urlencoding::{encode, decode};
#[allow(unused_imports)]
use log::{info, trace, warn, debug};
//use serde::ser::{Serialize, SerializeStruct, Serializer};
#[allow(unused_imports)]
use std::{thread, env, path::PathBuf};
//use std::sync::mpsc::channel;
use serde_json::json;
use futures::{future::{ok, Either}, executor};
use fancy_regex::Regex;
use time::Duration;

mod video;
mod music;
mod work_queue;
mod cli;
mod db;
mod user;
mod auth;
mod session;
use work_queue::*;
use video::*;
use music::*;
use user::User;
use db::{Db};
use session::Session;

#[post("/addVideo/{url}")]
async fn add_video(c: web::Data<Client>, q: web::Data<WorkQueue<Work>>, session: web::Data<Session>, url: web::Path<String>) -> Result<HttpResponse> {
    let mut v = Video::new(decode(&*url.into_inner()).unwrap());
    let res = Video::get_meta_data(&c, &v).await?;
    v.meta = Some(res);
    let user = session.get_user().unwrap();
    q.into_inner().add_work(Work::Download((user, v)));
    Ok(
        HttpResponse::Ok()
        .json(json!({ "result": "Success" }))
    )
}

#[get("/music/{playlist}")]
async fn get_music(session: web::Data<Session>, _playlist: web::Path<String>) -> Result<HttpResponse> {
    let user = session.get_user().unwrap();
    let mut music = user.music().clone();
    music.retain(|_track| {
        //TODO: filter based on playlist
        true
    });
    Ok(
        HttpResponse::Ok()
        .json(json!({ "result": "Success", "music": music }))
    )
}

#[get("/queue")]
async fn queue(q: web::Data<WorkQueue<Work>>) -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
        .json(
            q.into_inner().retain(|e| {
                match e {
                    Work::Download(_) => true,
                    _ => false
                }
            }).clone()
        )
    )
}

#[put("/register")]
async fn register(db: web::Data<Db>, data: web::Json<auth::Register>) -> Result<HttpResponse> {
    match auth::register(db.get_ref(), data.into_inner()).await {
        Ok(res) => {
            Ok(
                HttpResponse::Ok()
                .json(json!({ "result": res }))
            )
        },
        Err(e) => {
            Ok(
                HttpResponse::InternalServerError()
                .json(json!({ "result": e.to_string() }))
            )
        }
    }
}

#[post("/login")]
async fn login(db: web::Data<Db>, config: web::Data<cli::Args>, data: web::Json<auth::Login>) -> Result<HttpResponse> {
    match auth::login(db.get_ref(), data.into_inner(), &config.jwt_secret).await {
        Ok(token) => {
            let cookie = Cookie::build("token", &token)
                .max_age(Duration::days(90))
                .same_site(SameSite::Lax)
                .path("/")
                .http_only(true)
                .finish();
            let mut resp = HttpResponse::Ok()
                .json(json!({ "result": "success" }));
            resp.add_cookie(&cookie).unwrap();
            Ok(resp)
        },
        Err(e) => {
            Ok(
                HttpResponse::Ok()
                .json(json!({ "result": "failed", "error": e.to_string() }))
            )
        }
    }
}

#[get("/logout")]
async fn logout(req: HttpRequest, db: web::Data<Db>, config: web::Data<cli::Args>) -> Result<HttpResponse> {
    match req.headers().get(http::header::AUTHORIZATION) {
        Some(val) => {
            let token = val.to_str().unwrap();
            let token = token.replace("Bearer ", "");
            let claim = auth::validate_token(&token, &config.jwt_secret).await.unwrap();
            auth::logout(db.get_ref(), claim.get("userId").unwrap().as_str().unwrap()).await.unwrap();
            let cookie = Cookie::build("token", "")
                .max_age(Duration::days(0))
                .same_site(SameSite::Lax)
                .path("/")
                .http_only(true)
                .finish();
            let mut resp = HttpResponse::Ok()
                .json(json!({ "result": "success" }));
            resp.add_cookie(&cookie).unwrap();
            Ok(resp)
        },
        None => {
            Ok(
                HttpResponse::Ok()
                .json(json!({ 
                    "result": "failed",
                    "error": "JWT Token not set!"
                }))
            )
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = cli::Args::get_args();
    env::set_var("RUST_LOG", &("actix_web=".to_string() + &config.v_lvl) );
    env_logger::init();

    let q: WorkQueue<Work> = WorkQueue::new();
    let mut threads = Vec::new();
    for _ in 0..config.workers {
        let tq = q.clone();
        let db = Db::new(&config.dbhost, &config.dbuser, &config.dbpwd, &config.dbdb);
        let path = String::from(&config.download_path);
        let handle = thread::spawn(move || {
            loop {
                if let Some(work) = tq.get_work() {
                    match work {
                        Work::Download(data) => {
                            let (user, video) = data;
                            let file = video.download(&path).unwrap_or("Error".into());
                            let mut track = Track::new(file.as_str(), &video.link);
                            *track.status_mut() = TrackStatus::DownloadFinished();
                            executor::block_on(add_track(&db, user.id().unwrap(), &mut track)).unwrap();
                            println!("Download: {:?}", track);
                            tq.add_work(Work::Tag((user, track)));
                            debug!("OUTPUT: {}", file);
                        },
                        Work::Tag(data) => {
                            let (user, mut track) = data;
                            //TODO: Tag the musicfile
                            *track.status_mut() = TrackStatus::TaggingFinished();
                            println!("Tag: {:?}", track);
                            executor::block_on(update_track(&db, user.id().unwrap(), &track)).unwrap();
                            tq.add_work(Work::Normalize((user, track)));
                        },
                        Work::Normalize(data) => {
                            let (user, mut track) = data;
                            //TODO: Normalize the audiofile
                            println!("Normalize: {:?}", track);
                            *track.status_mut() = TrackStatus::Finished();
                            executor::block_on(update_track(&db, user.id().unwrap(), &track)).unwrap();
                        },
                    }
                }
                std::thread::yield_now();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });
        threads.push(handle);
    }
    
    // Actix
    let c = config.clone();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let db = Db::new(&c.dbhost, &c.dbuser, &c.dbpwd, &c.dbdb);
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .data(c.clone())
            .data(q.clone())
            .data(db)
            .data(Session::new())
            .data(Client::default())
            .wrap_fn(|mut req, srv| {
                // Transform Cookie into Authorization header
                if let Some(val) = req.headers().get(http::header::COOKIE) {
                    let val = val.to_owned();
                    let vals = val.to_str().unwrap().split(';');
                    for v in vals {
                        let c = Cookie::parse(v).unwrap();
                        if c.name() == "token" {
                            let token = c.value().to_owned();
                            req.headers_mut().insert(http::header::AUTHORIZATION, HeaderValue::from_str(&("Bearer ".to_owned() + &token[..])).unwrap());
                        }
                    }
                }

                // Redirect Rules
                match req.headers().get(http::header::AUTHORIZATION) {
                    Some(_) => {
                        debug!("AUTHORIZED");
                        if req.path() == "/" {
                            return Either::Right(ok(req.into_response(
                                HttpResponse::PermanentRedirect()
                                .header(http::header::LOCATION, "/app/home")
                                .finish()
                                .into_body(),
                            )));
                        }
                        let re = Regex::new(r"^\/auth(?!\/logout\/?).*$").unwrap();
                        if re.is_match(req.path()).unwrap() {
                            return Either::Right(ok(req.into_response(
                                HttpResponse::PermanentRedirect()
                                .header(http::header::LOCATION, "/")
                                .finish()
                                .into_body(),
                            )));
                        }
                        Either::Left(srv.call(req))
                    },
                    None => {
                        debug!("UNAUTHORIZED");
                        if req.path() == "/" {
                            return Either::Right(ok(req.into_response(
                                HttpResponse::PermanentRedirect()
                                .header(http::header::LOCATION, "/auth/login")
                                .finish()
                                .into_body(),
                            )));
                        }
                        let re = Regex::new(r"^(\/api.*|\/auth\/login\/?|\/auth\/register\/?|\/auth\/assets.*)$").unwrap();
                        if !re.is_match(req.path()).unwrap() {
                            return Either::Right(ok(req.into_response(
                                HttpResponse::PermanentRedirect()
                                .header(http::header::LOCATION, "/")
                                .finish()
                                .into_body(),
                            )));
                        }
                        Either::Left(srv.call(req))
                    },
                }
                //let cookies = req.headers().get("Cookie").unwrap().to_str().unwrap().to_string();
                //let session = req.app_data::<Session>().unwrap();
                //println!("{}", session.get::<String>("tet").unwrap().unwrap());
            })
            .service(web::scope("/auth")
                .service(login)
                .service(logout)
                .service(register)
                .service(fs::Files::new("/assets", "frontend/soundloop/dist/assets").show_files_listing())
                .service(fs::Files::new("/login", "frontend/soundloop/dist/login").index_file("login.html"))
                .service(fs::Files::new("/register", "frontend/soundloop/dist/login").index_file("register.html"))
            )
            .service(web::scope("/api")
                .service(web::scope("/v1")
                    .service(web::scope("/user")
                        .wrap(HttpAuthentication::bearer(auth::validate_handler))
                        .service(add_video)
                        .service(queue)
                        .service(get_music)
                    )
                )
            )
            .service(web::scope("/app")
                .wrap(HttpAuthentication::bearer(auth::validate_handler))
                .service(fs::Files::new("/assets", "frontend/soundloop/dist/assets").show_files_listing())
                .service(fs::Files::new("/home", "frontend/soundloop/dist/main").index_file("index.html"))
            )
    })
    .bind((config.ip.as_str(), config.port))?
        .run()
        .await
}


