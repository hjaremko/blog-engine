use crate::model::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: i32,
    pub date: String,
    pub title: String,
    pub author: User,
    pub content: String,
}

impl Post {
    // pub fn new(title: &str, content: &str, user: User) -> Self {
    //     Post {
    //         id: 0,
    //         date: "aaa".to_string(),
    //         title: title.to_string(),
    //         author: user,
    //         content: content.to_string(),
    //     }
    // }
}
