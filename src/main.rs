use actix_cors::Cors;
#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, Error, client::Client};
use actix_files as fs;
#[allow(unused_imports)]
use urlencoding::{encode, decode};
#[allow(unused_imports)]
use log::{info, trace, warn};
//use serde::ser::{Serialize, SerializeStruct, Serializer};
#[allow(unused_imports)]
use std::{thread, env};
//use std::sync::mpsc::channel;
use serde::{Deserialize, Serialize};

mod video;
mod work_queue;
mod cli;
use work_queue::*;
use video::*;

#[derive(Clone, Serialize, Deserialize)]
struct Resp {
    result: String,
}

#[post("/addVideo/{url}")]
async fn add_video(c: web::Data<Client>, q: web::Data<WorkQueue<Work>>, url: web::Path<String>) -> Result<HttpResponse> {
    let mut v = Video::new(decode(&*url.into_inner()).unwrap());
    let res = Video::get_meta_data(&c, &v).await?;
    v.meta = Some(res);
    q.into_inner().add_work(Work::Download(v));
    Ok(
        HttpResponse::Ok()
        .json(Resp { result: "Success".into() })
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
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .data(Client::default())
            .data(q.clone())
            .service(add_video)
            .service(queue)
            .service(fs::Files::new("/", "frontend/soundloop/dist").show_files_listing().index_file("index.html"))
    })
    .bind((config.ip.as_str(), config.port))?
        .run()
        .await
}


