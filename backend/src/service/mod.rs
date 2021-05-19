use crate::model::{Post, User, Rights, NewPostRequest};
use crate::repository::{PostsRepository, UserRepository};

pub struct UserService {}

impl UserService {
    pub fn get_all() -> Vec<User> {
        UserRepository::get_all().unwrap()
    }

    pub fn get(id: i32) -> User {
        User {
            id,
            name: "User".to_string(),
            rights: Rights::Common,
        }
    }
}


pub struct PostsService {}

impl PostsService {
    pub fn get_all() -> Vec<Post> {
        PostsRepository::get_all().unwrap()
    }

    pub fn add_post(request: NewPostRequest) -> Post {
        let author = UserService::get(request.author_id);

        let post = Post {
            id: PostsService::get_all().len() as i32,
            date: "today".to_string(),
            title: request.title,
            author,
            content: request.content,
        };


        PostsRepository::add_post(&post).unwrap();
        post
    }
}
