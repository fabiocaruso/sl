use actix_web::{Error, client::Client};
#[allow(unused_imports)]
use log::{info, trace, warn, error};
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use urlencoding::{encode, decode};
use fancy_regex::Regex;
use anyhow::{Result, bail};

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

    pub fn download(&self, path: &str) -> Result<String> {
        let child = Command::new("youtube-dl")
            .args(&[
                "-f", 
                "best", 
                "-o", 
                &[path, "/%(title)s.%(ext)s"].concat(), 
                "--extract-audio", 
                "--no-playlist", 
                "--audio-format", 
                "mp3", 
                &self.link
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = child.wait_with_output()?;
        if let Some(code) = output.status.code() {
            match code {
                0 => {
                    let out = std::str::from_utf8(&output.stdout)?;
                    let re = Regex::new(r"(\[ffmpeg\] [dD]estination: )(.*)\n")?;
                    if let Some(caps) = re.captures(out)? {
                        if let Some(file) = caps.get(2) {
                            info!("Download finished");
                            return Ok(file.as_str().into());
                        }
                        bail!("Can't extract filename!");
                    }
                    bail!("Can't extract filename!");
                },
                1 => {
                    let err = std::str::from_utf8(&output.stderr)?;
                    //TODO: Check for new version of youtube-dl -> upgrade -> retry
                    let msg: String = (&err[7..]).to_owned();
                    bail!(msg);
                },
                _ => bail!("Command failed with exit code {}!", code),
            }
        }
        bail!("Can't extract exit code!");
    }

}
