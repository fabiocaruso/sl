use actix_web::{Error, client::Client};
#[allow(unused_imports)]
use log::{info, trace, warn};
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use urlencoding::{encode, decode};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct VideoMeta {
    pub title: String,
    #[serde(rename(serialize = "thumbnail", deserialize = "thumbnail_url"))]
    pub thumbnail: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Video {
    pub link: String,
    pub meta: Option<VideoMeta>,
}

impl Video {

    pub fn new(url: String) -> Video {
        let v = Video {
            link: url,
            meta: None,
        };
        v
    }

    pub async fn get_meta_data(c: &Client, v: &Video) -> Result<VideoMeta, Error> {
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
