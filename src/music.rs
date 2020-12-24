use std::io;
use serde::{Deserialize, Serialize};
use serde_json::{json};
use super::{db::*};
use couchbase::{QueryOptions};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "code")]
pub enum TrackStatus {
    New(),
    DownloadFinished(),
    TaggingFinished(),
    NormalizeFinished(),
    DownloadError(u8),
    TaggingError(u8),
    NormalizeError(u8),
    Finished(),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackTags {
    title: String,
    album: Album,
    artist: Vec<Artist>,
    tracknr: u8,
    disk: u8,
    genre: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: Option<String>,
    pub link: String,
    pub file: String,
    pub status: TrackStatus,
    pub meta: Option<TrackTags>,
}

pub async fn add_track(db: &Db, user_id: &str, track: &mut Track) -> Result<String, io::Error> {
    //TODO: Get the newly generated UUID from the query
    let options = QueryOptions::default().named_parameters(
        json!({
            "id": user_id,
            "track": track,
        })
    );
    match db.query(Query{ n1ql: N1QL::ADD_TRACK.into(), options}).await {
        Ok(mut result) => {
            match result.pop() {
                Some(e) => {
                    match e {
                        QueryResult::Json(v) => {
                            *track.id_mut() = Some(v.get("id").unwrap().as_str().unwrap().to_string());
                            Ok("Success!".into())
                        }
                        _ => {
                            Err(io::Error::new::<String>(io::ErrorKind::Other, "Unexpected result!".into()))
                        }
                    }
                },
                None => {
                    Err(io::Error::new::<String>(io::ErrorKind::Other, "Cannot extract id!".into()))
                }
            }
        },
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }
    }
}

pub async fn update_track(db: &Db, user_id: &str, track: &Track) -> Result<String, io::Error> {
    let options = QueryOptions::default().named_parameters(
        json!({
            "id": user_id,
            "trackid": track.id().unwrap(),
            "status": track.status(),
        })
    );
    match db.query(Query{ n1ql: N1QL::UPDATE_TRACK.into(), options}).await {
        Ok(_) => {
            Ok("Success!".into())
        },
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }
    }
}

impl Track {
    
    pub fn new(file: &str, link: &str) -> Self {
        Self {
            id: None,
            link: link.to_owned(),
            file: file.to_owned(),
            status: TrackStatus::New(),
            meta: None,
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn id_mut(&mut self) -> &mut Option<String> {
        &mut self.id
    }

    pub fn status(&self) -> &TrackStatus {
        &self.status
    }

    pub fn status_mut(&mut self) -> &mut TrackStatus {
        &mut self.status
    }

    pub fn meta(&self) -> Option<&TrackTags> {
        self.meta.as_ref()
    }

    pub fn fetch_meta(&self) {
        
    }

}
