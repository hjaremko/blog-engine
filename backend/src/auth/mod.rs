use crate::model::Rights;
use crate::service::UserService;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Outcome, Request};
use serde::{Deserialize, Serialize};
use std::io;
use std::str::FromStr;

pub struct Credentials {
    pub login: String,
    pub rights: Rights,
}

impl<'a, 'r> FromRequest<'a, 'r> for Credentials {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                println!("{}", token);

                if let Some(c) = auth_token(&token) {
                    Outcome::Success(c)
                } else {
                    Outcome::Failure((Status::Unauthorized, ()))
                }
            }
            // token does not exist
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    login: String,
    password: String,
    role: String,
    exp: usize,
}

const JWT_SECRET: &[u8; 6] = b"secret";

pub fn create_jwt(login: &str, pass: &str, role: &Rights) -> Result<String, io::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        login: login.to_owned(),
        password: pass.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    let r = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap();
    Ok(r)
    // .map_err(|_| Error::JWTTokenCreationError)
}

pub fn auth_token(token: &str) -> Option<Credentials> {
    let parts = token.split_ascii_whitespace().collect::<Vec<&str>>();

    if parts.len() != 2 {
        return None;
    }

    if parts.get(0).unwrap() != &"Bearer" {
        return None;
    }

    return auth_jwt(parts.get(1).unwrap());
}

pub fn auth_jwt(token: &str) -> Option<Credentials> {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    );

    if let Err(_) = decoded {
        return None;
    }

    let decoded = decoded.unwrap();
    let role = Rights::from_str(&decoded.claims.role).unwrap();
    let login = &decoded.claims.login;
    let password = &decoded.claims.password;

    let user = UserService::get_by_login(login);
    println!("{}, {}, {}", role.to_string(), login, password);

    let valid = user.login == *login && user.password == *password;

    if valid {
        Some(Credentials {
            login: login.to_string(),
            rights: role,
        })
    } else {
        None
    }
}
