use std::{io, time::{SystemTime, UNIX_EPOCH}};
use sha3::{Digest, Sha3_256};
use actix_web::{web::Data, Error, dev::ServiceRequest};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use jsonwebtokens as jwt;
use jwt::{Algorithm, AlgorithmID, Verifier, encode};
use couchbase::{QueryOptions};
use super::{cli::Args, db::*, Session, user::fetch_user};
#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use serde_json::{json, value::Value};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Register {
    pub name: String,
    pub first_name: String,
    pub email: String,
    pub password: String,
}

pub async fn validate_handler(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req.app_data::<Data<Args>>().unwrap().get_ref();
    match validate_token(credentials.token(), &config.jwt_secret).await {
        Ok(claim) => {
            let db = req.app_data::<Data<Db>>().unwrap().get_ref();
            let session = req.app_data::<Data<Session>>().unwrap().get_ref().clone();
            //TODO: Handle no claim key error
            let id = claim.get("userId").unwrap().as_str().unwrap();
            let sid = claim.get("session").unwrap().as_str().unwrap();
            let user = fetch_user(db, id, sid).await.unwrap();
            session.set_user(user);
            Ok(req)
        },
        Err(_) => {
            let config = req.app_data::<Config>()
                .map(|data| data.clone())
                .unwrap_or_else(Default::default);
            Err(AuthenticationError::from(config).into())
        },
    }
}

pub async fn validate_token(token: &str, secret: &str) -> Result<Value, jwt::error::Error> {
    let alg = Algorithm::new_hmac(AlgorithmID::HS256, secret)?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let verifier = Verifier::create()
        //.issuer("http://some-auth-service.com")
        .audience("user")
        .subject("Soundloop")
        //.leeway(5)
        .build()?;
    let claims: Value = verifier.verify_for_time(token, &alg, now)?.claims;
    Ok(claims)
}

pub fn hash_pw(pw: &str) -> String {
    hex::encode(Sha3_256::digest(pw.as_bytes()).as_slice())
}

//TODO: AuthError wrapper for std::error:Error
pub async fn login(db: &Db, data: Login, secret: &str) -> Result<String, impl std::error::Error> {
    let options = QueryOptions::default().named_parameters(
        json!({
            "email": data.email,
            "hash": hash_pw(&data.password),
            "session": Uuid::new_v4().to_hyphenated().to_string(),
        })
    );
    let mut result = db.query(Query{ n1ql: N1QL::LOGIN.into(), options}).await.unwrap();
    match result.pop() {
        Some(r) => match r {
            QueryResult::User(u) => {
                // Build JWT Token
                let jwt_lifetime = 7_776_000; // 3 Months
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let alg = Algorithm::new_hmac(AlgorithmID::HS256, secret).unwrap();
                let header = json!({ 
                    "alg": alg.name(),
                });
                let claims = json!({
                    "userId": u.id().unwrap(),
                    "session": u.session().unwrap(),
                    //"iss": "Soundloop",
                    "sub": "Soundloop",
                    "aud": "user",
                    "exp": now + jwt_lifetime,
                    "nbf": now,
                    "iat": now,
                });
                let token = encode(&header, &claims, &alg).unwrap();
                Ok(token)
            },
            _ => Err(io::Error::new(io::ErrorKind::Other, "Database error!")),
        },
        None => Err(io::Error::new(io::ErrorKind::Other, "User not found!")),
    }
}

//TODO: AuthError wrapper for std::error:Error
pub async fn register(db: &Db, data: Register) -> Result<String, impl std::error::Error> {
    let options = QueryOptions::default().named_parameters(
        json!({
            "type": "user",
            "name": data.name,
            "first_name": data.first_name,
            "email": data.email,
            "hash": hash_pw(&data.password),
            "role": "user",
            "session": ""
        })
    );
    //TODO: Bad error catching
    //TODO: Check the number of results (if 0 then there is already a email like this in the db)
    match db.query(Query{ n1ql: N1QL::REGISTER.into(), options}).await {
        Ok(_) => {
            Ok("Success!".into())
        },
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }
    }
}

pub async fn logout(db: &Db, id: &str) -> Result<String, impl std::error::Error> {
    let options = QueryOptions::default().named_parameters(
        json!({
            "id": id,
        })
    );
    //TODO: Bad error catching
    match db.query(Query{ n1ql: N1QL::LOGOUT.into(), options}).await {
        Ok(_) => {
            Ok("Success!".into())
        },
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }
    }
}
