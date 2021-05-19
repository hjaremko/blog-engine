#![feature(proc_macro_hygiene, decl_macro)]

mod model;
mod repository;
mod service;
mod controller;

#[macro_use]
extern crate rocket;

use rusqlite::{Connection, Result};
use crate::repository::{PostsRepository, UserRepository};
use std::rc::Rc;
use crate::service::PostsService;
use crate::controller::*;
use crate::db::CONN;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub mod db {
    use rusqlite::Connection;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    pub static CONN: Lazy<Mutex<Connection>> = Lazy::new(|| Mutex::new(Connection::open("blog.db").unwrap()));
}

fn main() {
    UserRepository::init_tables().unwrap();
    PostsRepository::init_tables().unwrap();

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api", routes![all_posts, new_post])
        .launch();
}