#![feature(proc_macro_hygiene, decl_macro)]

mod controller;
mod model;
mod repository;
mod service;

#[macro_use]
extern crate rocket;

use crate::controller::*;
use crate::db::CONN;
use crate::repository::{PostsRepository, UserRepository};
use crate::service::PostsService;
use rocket::response::{NamedFile, Redirect};
use rusqlite::{Connection, Result};
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[get("/")]
fn index() -> Redirect {
    Redirect::permanent("/index.html")
}

#[get("/<file..>")]
fn build_dir(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("../frontend/frontend/build/").join(file))
}

pub mod db {
    use once_cell::sync::Lazy;
    use rusqlite::Connection;
    use std::sync::Mutex;

    pub static CONN: Lazy<Mutex<Connection>> =
        Lazy::new(|| Mutex::new(Connection::open("blog.db").unwrap()));
}

fn main() {
    UserRepository::init_tables().unwrap();
    PostsRepository::init_tables().unwrap();

    rocket::ignite()
        .mount("/", routes![index, build_dir])
        .mount("/api", routes![all_posts, new_post])
        .launch();
}
