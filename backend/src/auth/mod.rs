use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::Utc;
use crate::model::Rights;
use std::io;
use serde::{Deserialize, Serialize};
use rocket::request::FromRequest;
use std::convert::Infallible;
use rocket::{Request, request, Outcome};
use rocket::http::Status;
use std::str::FromStr;

pub struct Token(String);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                println!("{}", token);
                // check validity
                Outcome::Success(Token(token.to_string()))
            }
            // token does not exist
            // None => Outcome::Failure((Status::Unauthorized, Infallible::from(401)))
            None => Outcome::Failure((Status::Unauthorized, ()))
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

pub fn auth_jwt(token: &str) {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    ).unwrap();

    let role = Rights::from_str(&decoded.claims.role).unwrap();
    let login = &decoded.claims.login;
    let password = &decoded.claims.password;
    let exp = &decoded.claims.exp;

    let now = Utc::now()
        .timestamp();

    println!("{}, {}, {}", role.to_string(), login, password);
    println!("{} < {}", exp, now);
    println!("{}", exp < &(now as usize));
}
