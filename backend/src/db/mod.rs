use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::Mutex;

pub static CONN: Lazy<Mutex<Connection>> =
    Lazy::new(|| Mutex::new(Connection::open("blog.db").unwrap()));
