#![feature(proc_macro_hygiene, decl_macro)]

mod controller;
mod model;
mod repository;
mod service;

#[macro_use]
extern crate rocket;

use crate::controller::*;
use crate::db::CONN;
use crate::repository::{PostsRepository, UserRepository, CommentsRepository};
use crate::service::PostsService;
use rocket::response::{NamedFile, Redirect};
use rusqlite::{Connection, Result};
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Method;

#[get("/")]
fn index() -> Redirect {
    Redirect::permanent("/index.html")
}

#[get("/<file..>", rank = 2)]
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
    CommentsRepository::init_tables().unwrap();


    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::ignite()
        .mount("/", routes![index, build_dir])
        .mount("/api", routes![all_posts, posts_page, new_post, get_comments])
        .attach(cors.to_cors().unwrap())
        .launch();
}
