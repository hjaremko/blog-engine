use serde::{Serialize, Deserialize};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

#[derive(Debug, Serialize, Deserialize)]
pub enum Rights {
    Administrator,
    Moderator,
    Common,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub rights: Rights,
}

#[derive(Debug, Serialize)]
pub struct Comment {
    id: i32,
    author: User,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPostRequest {
    pub title: String,
    pub author_id: i32,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
            },
            content: content.to_string(),
        }
    }
}
