use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Rights {
    Administrator,
    Common,
}

impl ToString for Rights {
    fn to_string(&self) -> String {
        match self {
            Rights::Administrator => { "ADMIN" }
            Rights::Common => { "USER" }
        }.to_string()
    }
}

impl FromStr for Rights {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADMIN" => Ok(Rights::Administrator),
            "USER" => Ok(Rights::Common),
            _ => Err(())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    #[serde(skip)]
    pub login: String,
    #[serde(skip)]
    pub password: String,
    pub name: String,
    pub rights: Rights,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: i32,
    pub date: String,
    pub author: User,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPostRequest {
    pub title: String,
    // pub author_id: i32,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub login: String,
    pub password: String,
    pub name: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: i32,
    pub date: String,
    pub title: String,
    pub author: User,
    pub content: String,
}

impl Post {
    pub fn new(title: &str, content: &str) -> Self {
        Post {
            id: 0,
            date: "aaa".to_string(),
            title: title.to_string(),
            author: User {
                id: 0,
                name: "user".to_string(),
                rights: Rights::Administrator,
                password: "".to_string(),
                login: "".to_string()
            },
            content: content.to_string(),
        }
    }
}
