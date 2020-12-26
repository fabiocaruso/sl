use super::{Track, db::*};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize};
use serde_json::{json};
use couchbase::{QueryOptions};
use anyhow::{Result, bail};

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct User {
    id: Option<String>,
    first_name: Option<String>,
    name: Option<String>,
    email: Option<String>,
    role: Option<String>,
    hash: Option<String>,
    session: Option<String>,
    music: Vec<Track>,
}

impl Serialize for User {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("User", 5)?;
        s.serialize_field("type", "user")?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("first_name", &self.first_name)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("hash", &self.hash)?;
        s.serialize_field("session", &self.session)?;
        s.serialize_field("music", &self.music)?;
        s.end()
    }

}

pub async fn fetch_user(db: &Db, id: &str, sid: &str) -> Result<User> {
    let options = QueryOptions::default().named_parameters(
        json!({
            "id": id,
            "session": sid,
        })
    );
    let mut result = db.query(Query{ n1ql: N1QL::GET_USER_BY_ID.into(), options}).await?;
    if let Some(e) = result.pop() {
        if let QueryResult::User(u) = e {
            return Ok(u)
        }
        bail!("Query Error!");
    }
    bail!("User not found!");
}

impl User {
    
    pub fn _new() -> Self {
        Self {
            id: None,
            first_name: None,
            name: None,
            email: None,
            role: None,
            hash: None,
            session: None,
            music: Vec::new(),
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn session(&self) -> Option<&String> {
        self.session.as_ref()
    }

    pub fn music(&self) -> &Vec<Track> {
        self.music.as_ref()
    }
    
    //TODO: INSERT user into database and get id if user is constructed

}
