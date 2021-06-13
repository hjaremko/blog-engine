use crate::auth::Credentials;
use crate::model::{NewPostRequest, Rights};
use crate::service::{CommentsService, PostsService, UserService};
use rocket::response::content;
use rocket::response::content::Json;
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
pub fn new_post(
    credentials: Credentials,
    input: rocket_contrib::json::Json<NewPostRequest>,
) -> Result<Json<String>, Unauthorized<String>> {
    let request = input.into_inner();

    if credentials.rights != Rights::Administrator {
        return Err(Unauthorized(Option::from(
            "You are not allowed to add new posts".to_string(),
        )));
    }

    let user = UserService::get_by_login(&credentials.login).unwrap();
    let post = PostsService::add_post(request.title, request.content, user);
    let json = serde_json::to_string(&post).unwrap();
    Ok(content::Json(json))
}

#[get("/posts/comments/<post_id>")]
pub fn get_comments(post_id: usize) -> content::Json<String> {
    let comment = CommentsService::get_all(post_id);
    let json = serde_json::to_string(&comment).unwrap();
    content::Json(json)
}

#[post("/posts/comments/<post_id>", format = "plain", data = "<input>")]
pub fn new_comment(
    credentials: Credentials,
    post_id: usize,
    input: String,
) -> content::Json<String> {
    let user = UserService::get_by_login(&credentials.login).unwrap();
    let comment = CommentsService::create(post_id, input, user);
    let json = serde_json::to_string(&comment).unwrap();
    content::Json(json)
}
