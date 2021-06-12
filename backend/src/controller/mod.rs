use crate::model::{NewPostRequest, Post, Rights, User};
use crate::service::PostsService;
use crate::service::CommentsService;
use rocket::response::content;
use rocket::response::content::Json;
use rocket::Request;

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
