use crate::model::{RegisterRequest, Rights, User};
use crate::repository::UserRepository;

pub struct UserService {}

impl UserService {}

impl UserService {
    pub fn get_all() -> Vec<User> {
        UserRepository::get_all().unwrap()
    }

    pub fn get(id: i32) -> User {
        User {
            id,
            name: "User".to_string(),
            rights: Rights::Common,
            password: "".to_string(),
            login: "".to_string(),
        }
    }

    pub fn get_by_login(login: &str) -> Option<User> {
        let users = UserRepository::get_all().unwrap();
        users.into_iter().find(|x| x.login == login)
    }

    pub fn create(request: RegisterRequest) -> User {
        UserRepository::create(request).unwrap();
        User {
            id: 0,
            login: "".to_string(),
            password: "".to_string(),
            name: "".to_string(),
            rights: Rights::Administrator,
        }
    }
}
