use crate::model::{NewPostRequest, Post, Rights, User, LoginRequest, RegisterRequest};
use crate::service::{PostsService, UserService};
use crate::service::CommentsService;
use rocket::response::content;
use rocket::response::content::Json;
use rocket::{Request, Response};
use crate::auth::{Token, create_jwt};
use rocket::http::Status;
use std::io::Cursor;
use rocket::response::status::Unauthorized;

#[get("/posts")]
pub fn all_posts() -> content::Json<String> {
    let posts = PostsService::get_all();
    let json = serde_json::to_string(&posts).unwrap();
    content::Json(json)
}


#[get("/posts?<page>&<limit>")]
pub fn posts_page(page: usize, limit: usize) -> content::Json<String> {
    let posts = PostsService::get_page(page, limit);
    let json = serde_json::to_string(&posts).unwrap();
    content::Json(json)
}


#[post("/posts", format = "json", data = "<input>")]
pub fn new_post(input: rocket_contrib::json::Json<NewPostRequest>) -> content::Json<String> {
    let request = input.into_inner();
    let post = PostsService::add_post(request);
    let json = serde_json::to_string(&post).unwrap();
    content::Json(json)
}

#[get("/posts/comments/<post_id>")]
pub fn get_comments(post_id: usize) -> content::Json<String> {
    let comment = CommentsService::get_all(post_id);
    let json = serde_json::to_string(&comment).unwrap();
    content::Json(json)
}

#[post("/login", format = "json", data = "<input>")]
pub fn login(input: rocket_contrib::json::Json<LoginRequest>) -> Result<content::Plain<String>, Unauthorized<String>> {
    let request = input.into_inner();
    let user = UserService::get_by_login(&request.login);

    if request.login == user.login && request.password == user.password {
        let jwt = create_jwt(&user.login, &user.password, &user.rights).unwrap();
        return Ok(content::Plain(jwt));
    }

    Err(Unauthorized(Option::from("Invalid login or password".to_string())))
}


#[post("/register", format = "json", data = "<input>")]
// pub fn login(token: Token) {
pub fn register(input: rocket_contrib::json::Json<RegisterRequest>) -> content::Json<String> {
    let request = input.into_inner();
    let user = UserService::create(request);
    let json = serde_json::to_string(&user).unwrap();
    content::Json(json)
}
