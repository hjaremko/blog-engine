use crate::model::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: i32,
    pub date: String,
    pub author: User,
    pub content: String,
}
