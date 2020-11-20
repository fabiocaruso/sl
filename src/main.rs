use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, Error, client::Client};
use actix_files as fs;
use urlencoding::{encode, decode};
use log::{info, trace, warn};
use serde::{Deserialize, Serialize};
//use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::process::{Command, Stdio};
use std::{thread, env};
//use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

static mut VERBOSE_LVL: u8 = 0;

#[derive(Clone, Serialize, Deserialize)]
struct WorkQueue<T: Send> {
    #[serde(rename(serialize = "queue", deserialize = "inner"))]
    inner: Arc<Mutex<VecDeque<T>>>,
}

impl<T: Send + PartialEq + Clone + Serialize> WorkQueue<T> {

    fn new() -> Self { 
        Self { inner: Arc::new(Mutex::new(VecDeque::new())) } 
    }

    fn get_work(&self) -> Option<T> {
        if let Ok(mut q) = self.inner.lock() {
            q.pop_front()
        } else {
            panic!("WorkQueue::get_work() tried to lock a poisoned mutex")
        }
    }

    fn add_work(&self, work: T) -> usize {
        if let Ok(mut q) = self.inner.lock() {
            if !q.contains(&work) {
                q.push_back(work);
            }
            q.len()
        } else {
            panic!("WorkQueue::get_work() tried to lock a poisoned mutex")
        }
    }

}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct VideoMeta {
    title: String,
    #[serde(rename(serialize = "thumbnail", deserialize = "thumbnail_url"))]
    thumbnail: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Video {
    link: String,
    meta: Option<VideoMeta>,
}

impl Video {

    pub fn new(url: String) -> Video {
        let v = Video {
            link: url,
            meta: None,
        };
        v
    }

    async fn get_meta_data(c: &Client, v: &Video) -> Result<VideoMeta, Error> {
        println!("addVideo1");
        let res = c
            .get(&["https://www.youtube.com/oembed?format=json&url=", &encode(&v.link)].concat())
            .send()
            .await?
            .body()
            .await?;
        let r = res.clone();
        let s = std::str::from_utf8(r.as_ref()).unwrap();
        let v: VideoMeta = serde_json::from_str(s).unwrap();
        Ok(v)
    }

    pub fn download(&self, path: &str) -> Result<String, String> {
        println!("DOWNLOAD STARTED");
        let child = Command::new("youtube-dl")
                .args(&["-f", "best", "-o", &[path, "/%(title)s.%(ext)s"].concat(), "--extract-audio", "--no-playlist", "--audio-format", "mp3", &self.link])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute process");
        let output = child
                .wait_with_output()
                .expect("failed to wait on child");
        match output.status.code() {
            Some(code) => match code {
                0 => match std::str::from_utf8(&output.stdout) {
                    Ok(out) => {
                        info!("Download finished");
                        Ok(out.into())
                    },
                    Err(_) => Err("Can't get output of youtube-dl!".into())
                },
                1 => match std::str::from_utf8(&output.stderr) {
                    Ok(out) => {
                        info!("Failed to download file!");
                        Err((&out[7..]).into())
                    },
                    Err(_) => Err("Can't get output of youtube-dl!".into())
                },
                _ => Err("".into()),
            },
            None => Err("".into()),
        }
    }

}

#[post("/addVideo/{url}")]
async fn add_video(c: web::Data<Client>, q: web::Data<WorkQueue<Video>>, url: web::Path<String>) -> Result<String, Error> {
    let mut v = Video::new(decode(&*url.into_inner()).unwrap());
    let res = Video::get_meta_data(&c, &v).await?;
    v.meta = Some(res);
    q.into_inner().add_work(v);
    Ok("Success".into())
}

#[get("/queue")]
async fn queue(q: web::Data<WorkQueue<Video>>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(q.into_inner().clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Args
    let matches = clap::App::new("soundloop")
        .version("1.0")
        .author("lol <lol@lol>")
        .about("Soundloop Server")
        .arg(clap::Arg::new("ip")
            .short('i')
            .long("ip")
            .value_name("IP")
            .about("Sets the bind ip of the server")
            .takes_value(true))
        .arg(clap::Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .about("Sets the bind port of the server")
            .takes_value(true))
        .arg(clap::Arg::new("workers")
            .short('w')
            .long("workers")
            .value_name("WORKERS")
            .about("Number of download worker threads to run in parallel.")
            .takes_value(true))
        .arg(clap::Arg::new("dir")
            .short('d')
            .long("dir")
            .value_name("DOWNLOAD_DIR")
            .about("Set the download directory.")
            .takes_value(true))
        .arg(clap::Arg::new("verbose")
            .short('v')
            .about("Sets the level of verbosity")
            .multiple(true))
        .get_matches();
    let download_path = matches.value_of("dir").unwrap_or("downloads").to_owned();
    let ip = matches.value_of("ip").unwrap_or("0.0.0.0");
    let port = matches.value_of("port").unwrap_or("8000").parse::<u16>().unwrap_or(8000);
    let workers = matches.value_of("workers").unwrap_or("5").parse::<u8>().unwrap_or(5);
    unsafe {
        VERBOSE_LVL = matches.value_of("verbose").unwrap_or("0").parse::<u8>().unwrap_or(0);
    }

    let q: WorkQueue<Video> = WorkQueue::new();
    let mut threads = Vec::new();
    for _ in 0..workers {
        let tq = q.clone();
        let path = String::from(&download_path);
        let handle = thread::spawn(move || {
            loop {
                if let Some(work) = tq.get_work() {
                    let output = work.download(&path).unwrap_or("Error".into());
                    println!("OUTPUT: {}", output);
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
    .bind((ip, port))?
        .run()
        .await
}


