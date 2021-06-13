use crate::auth::{create_jwt, Credentials};
use crate::model::{LoginRequest, NewPostRequest, RegisterRequest, Rights};
use crate::service::CommentsService;
use crate::service::{PostsService, UserService};
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

    let user = UserService::get_by_login(&credentials.login);
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

#[post("/login", format = "json", data = "<input>")]
pub fn login(
    input: rocket_contrib::json::Json<LoginRequest>,
) -> Result<content::Plain<String>, Unauthorized<String>> {
    let request = input.into_inner();
    let user = UserService::get_by_login(&request.login);

    if request.login == user.login && request.password == user.password {
        let jwt = create_jwt(&user.login, &user.password, &user.rights).unwrap();
        return Ok(content::Plain(jwt));
    }

    Err(Unauthorized(Option::from(
        "Invalid login or password".to_string(),
    )))
}

#[post("/register", format = "json", data = "<input>")]
pub fn register(input: rocket_contrib::json::Json<RegisterRequest>) -> content::Json<String> {
    let request = input.into_inner();
    let user = UserService::create(request);
    let json = serde_json::to_string(&user).unwrap();
    content::Json(json)
}
