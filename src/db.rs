#[allow(unused_imports)]
use log::{info, trace, warn, error};
use couchbase::{Cluster, QueryOptions, CouchbaseError};
use futures::stream::StreamExt;
use serde::{Deserialize};
use serde_json::Value;
use super::User;

pub enum N1QL{}
impl N1QL {
    pub const LOGIN: &'static str = "UPDATE Soundloop SET `session`=$session WHERE `email`=$email AND `hash`=$hash RETURNING {meta().id, `first_name`, `name`, `email`, `role`, `session`, `music`} AS `User`";
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
                'session': $session,
                'music': []
            }
        ] AS u
        ON u.email = sl.email
        WHEN NOT MATCHED THEN INSERT (KEY UUID(),VALUE u)";
    pub const GET_USER_BY_ID: &'static str = "SELECT {meta().id, `first_name`, `name`, `email`, `role`, `session`, `music`} AS `User` FROM Soundloop AS sl WHERE meta().id=$id AND `session`=$session;";
    pub const LOGOUT: &'static str = "UPDATE Soundloop SET `session`='' WHERE meta().id=$id";
    pub const ADD_TRACK: &'static str = "
        UPDATE `Soundloop` AS sl
        SET sl.music = ARRAY_PUT(sl.music, OBJECT_PUT($track, 'id', UUID()))
        WHERE meta(sl).id = $id
        RETURNING sl.music[ARRAY_LENGTH(sl.music)-1] AS Json";
    pub const UPDATE_TRACK: &'static str = "
        UPDATE `Soundloop` AS sl
        SET sl.music[i].status = $status FOR i: t IN sl.music WHEN t.id = $trackid END
        WHERE meta(sl).id = $id;";
}

pub struct Db {
    conn: Cluster,
}

pub struct Query {
    pub n1ql: &'static str,
    pub options: QueryOptions,
}

#[derive(Debug, Deserialize)]
pub enum QueryResult {
    Json(Value),
    User(User),
    NoResult,
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
        let mut result = self.conn.query(q.n1ql, q.options).await?;
        let mut ret: Vec<QueryResult> = Vec::new();
        let mut iter = result.rows::<QueryResult>();
        for row in iter.next().await {
            match row {
                Ok(r) => {
                    match r {
                        QueryResult::NoResult => {},
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
