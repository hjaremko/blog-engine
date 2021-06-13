use crate::auth::create_jwt;
use crate::model::{LoginRequest, LoginResponse, RegisterRequest};
use crate::service::UserService;
use rocket::response::content;
use rocket::response::status::Unauthorized;

#[post("/login", format = "json", data = "<input>")]
pub fn login(
    input: rocket_contrib::json::Json<LoginRequest>,
) -> Result<content::Json<String>, Unauthorized<String>> {
    let err = Err(Unauthorized(Option::from(
        "Invalid login or password".to_string(),
    )));
    let request = input.into_inner();
    let user = UserService::get_by_login(&request.login);

    if user.is_none() {
        return err;
    }

    let user = user.unwrap();

    if request.login == user.login && request.password == user.password {
        let jwt = create_jwt(&user.login, &user.password, &user.rights).unwrap();
        let response = LoginResponse {
            token: jwt,
            rights: user.rights.to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return Ok(content::Json(response));
    }

    err
}

#[post("/register", format = "json", data = "<input>")]
pub fn register(input: rocket_contrib::json::Json<RegisterRequest>) -> content::Json<String> {
    let request = input.into_inner();
    let user = UserService::create(request);
    let json = serde_json::to_string(&user).unwrap();
    content::Json(json)
}
