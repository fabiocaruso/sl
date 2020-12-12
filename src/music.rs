use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    name: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    name: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MusicTags {
    title: String,
    album: Album,
    artist: Vec<Artist>,
    tracknr: u8,
    disk: u8,
    genre: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Music {
    pub file: String,
    pub meta: Option<MusicTags>,
}

impl Music {
    
    pub fn new(&self, file: String) -> Self {
        Self {
            file,
            meta: None,
        }
    }

    pub fn meta(&self) -> Option<MusicTags> {
        self.meta.clone()
    }

    pub fn fetch_meta(&self) {
        
    }

}
