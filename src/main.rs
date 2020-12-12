use actix_cors::Cors;
use actix_service::Service;
#[allow(unused_imports)]
use actix_web::{get, post, put, web, App, http, HttpResponse, HttpRequest, HttpServer, Responder, Result, Error, client::Client, middleware::Logger};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_files as fs;
#[allow(unused_imports)]
use urlencoding::{encode, decode};
#[allow(unused_imports)]
use log::{info, trace, warn};
//use serde::ser::{Serialize, SerializeStruct, Serializer};
#[allow(unused_imports)]
use std::{thread, env};
//use std::sync::mpsc::channel;
use serde_json::json;
use futures::future::{ok, Either, Ready};
use fancy_regex::Regex;

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
use db::Db;
use session::Session;

#[post("/addVideo/{url}")]
async fn add_video(c: web::Data<Client>, q: web::Data<WorkQueue<Work>>, url: web::Path<String>) -> Result<HttpResponse> {
    let mut v = Video::new(decode(&*url.into_inner()).unwrap());
    let res = Video::get_meta_data(&c, &v).await?;
    v.meta = Some(res);
    q.into_inner().add_work(Work::Download(v));
    Ok(
        HttpResponse::Ok()
        .json(json!({ "result": "Success" }))
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
    let res = match auth::login(db.get_ref(), data.into_inner(), &config.jwt_secret).await {
        Ok(token) => {
           json!({ "result": "success", "token": token })
        },
        Err(e) => {
           json!({ "result": e.to_string() })
        }
    };
    Ok(
        HttpResponse::Ok()
        .json(res)
    )
}

#[get("/logout")]
async fn logout(req: HttpRequest, db: web::Data<Db>, config: web::Data<cli::Args>) -> Result<HttpResponse> {
    match req.headers().get(http::header::AUTHORIZATION) {
        Some(val) => {
            let token = val.to_str().unwrap();
            let token = token.replace("Bearer ", "");
            let claim = auth::validate_token(&token, &config.jwt_secret).await.unwrap();
            auth::logout(db.get_ref(), claim.get("userId").unwrap().as_str().unwrap()).await.unwrap();
            Ok(
                HttpResponse::Ok()
                .json(json!({ 
                    "result": "success",
                }))
            )
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

    let q: WorkQueue<Work> = WorkQueue::new();
    let mut threads = Vec::new();
    for _ in 0..config.workers {
        let tq = q.clone();
        let path = String::from(&config.download_path);
        let handle = thread::spawn(move || {
            loop {
                if let Some(work) = tq.get_work() {
                    match work {
                        Work::Download(video) => {
                            let output = video.download(&path).unwrap_or("Error".into());
                            println!("OUTPUT: {}", output);
                        },
                        Work::Tag(_) => {
                        },
                        _ => {}
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
            .wrap_fn(|req, srv| {
                // Redirect Rules
                match req.headers().get(http::header::AUTHORIZATION) {
                    Some(_) => {
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
                        if req.path() == "/" {
                            return Either::Right(ok(req.into_response(
                                HttpResponse::PermanentRedirect()
                                .header(http::header::LOCATION, "/auth/login")
                                .finish()
                                .into_body(),
                            )));
                        }
                        let re = Regex::new(r"^(\/api.*|\/auth\/login\/?|\/auth\/register\/?)$").unwrap();
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
                .service(fs::Files::new("/login", "frontend/soundloop/dist").index_file("login.html"))
                .service(fs::Files::new("/register", "frontend/soundloop/dist").index_file("register.html"))
            )
            .service(web::scope("/api")
                .wrap(HttpAuthentication::bearer(auth::validate_handler))
                .service(web::scope("/v1")
                    .service(add_video)
                    .service(queue)
                )
            )
            .service(web::scope("/app")
                .wrap(HttpAuthentication::bearer(auth::validate_handler))
                .service(fs::Files::new("/home", "frontend/soundloop/dist").show_files_listing().index_file("index.html"))
            )
    })
    .bind((config.ip.as_str(), config.port))?
        .run()
        .await
}


