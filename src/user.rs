use super::db::{fetch_user};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    id: Option<String>,
    first_name: Option<String>,
    name: Option<String>,
    email: Option<String>,
    role: Option<String>,
    hash: Option<String>,
    session: Option<String>,
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
        s.end()
    }

}

impl User {
    
    pub fn new() -> Self {
        Self {
            id: None,
            first_name: None,
            name: None,
            email: None,
            role: None,
            hash: None,
            session: None,
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn session(&self) -> Option<&String> {
        self.session.as_ref()
    }
    
    //TODO: INSERT user into database and get id if user is constructed

}
