#[allow(unused_imports)]
use log::{info, trace, warn, error};
use couchbase::{Cluster, QueryOptions, CouchbaseError};
use futures::stream::StreamExt;
use serde::{Deserialize};
use serde_json::json;
use super::User;
use std::io;

pub enum N1QL{}
impl N1QL {
    pub const LOGIN: &'static str = "UPDATE Soundloop SET `session`=$session WHERE `email`=$email AND `hash`=$hash RETURNING {meta().id, `first_name`, `name`, `email`, `role`, `session`} AS `User`";
    pub const REGISTER: &'static str = "
        MERGE INTO `Soundloop` AS sl
        USING [
            {
                'type': $type,
                'email': $email,
                'name': $name,
                'first_name': $first_name,
                'hash': $hash,
                'role': $role,
                'session': $session
            }
        ] AS u
        ON u.email = sl.email
        WHEN NOT MATCHED THEN INSERT (KEY UUID(),VALUE u)";
    pub const GET_USER_BY_ID: &'static str = "SELECT {meta().id, `first_name`, `name`, `email`, `role`, `session`} AS `User` FROM Soundloop AS sl WHERE meta().id=$id AND `session`=$session;";
    pub const LOGOUT: &'static str = "UPDATE Soundloop SET `session`='' WHERE meta().id=$id";
}

pub struct Db {
    conn: Cluster,
}

pub struct Query {
    pub n1ql: String,
    pub options: QueryOptions,
}

#[derive(Debug, Deserialize)]
pub enum QueryResult {
    User(User),
    None,
}

pub async fn fetch_user(db: &Db, id: &str, sid: &str) -> Result<User, io::Error> {
    let options = QueryOptions::default().named_parameters(
        json!({
            "id": id,
            "session": sid,
        })
    );
    let mut result = db.query(Query{ n1ql: N1QL::GET_USER_BY_ID.into(), options}).await.unwrap();
    match result.pop() {
        Some(r) => match r {
            QueryResult::User(u) => Ok(u),
            QueryResult::None => Err(io::Error::new(io::ErrorKind::Other, "Database error!")),
        },
        None => Err(io::Error::new(io::ErrorKind::Other, "User not found!")),
    }
}

impl Db {

    pub fn new(host: &str, user: &str, password: &str, bucket: &str) -> Self {
        let conn = Cluster::connect(host, user, password);
        let bucket = conn.bucket(bucket);
        let _collection = bucket.default_collection();
        Self {
            conn,
        }
    }

    pub async fn query(&self, q: Query) -> Result<Vec<QueryResult>, CouchbaseError> {
        let mut result = self.conn.query(&q.n1ql, q.options).await?;
        let mut ret: Vec<QueryResult> = Vec::new();
        for row in result.rows::<QueryResult>().next().await {
            match row {
                Ok(r) => {
                    println!("query row: {:?}", r);
                    match r {
                        QueryResult::None => {}
                        _ => {
                            ret.push(r);
                        }
                    }
                },
                _ => {
                    error!("Can't parse the result of following query: {}", &q.n1ql);
                }
            }
        }
        Ok(ret)
    }

}
