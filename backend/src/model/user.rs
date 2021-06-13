use crate::model::Rights;
use serde::{Deserialize, Serialize};

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
