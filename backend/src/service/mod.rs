use crate::model::{Comment, Post, RegisterRequest, Rights, User};
use crate::repository::{CommentsRepository, PostsRepository, UserRepository};
use chrono::{DateTime, Utc};

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

pub struct PostsService {}

impl PostsService {
    pub fn get_all() -> Vec<Post> {
        PostsRepository::get_all().unwrap()
    }

    pub fn get_page(page_number: usize, posts_per_page: usize) -> Vec<Post> {
        let all_posts = PostsRepository::get_all().unwrap();
        let mut pages = all_posts.chunks(posts_per_page);

        let page = pages.nth(page_number);

        if let None = page {
            return vec![];
        }

        page.unwrap().into()
    }

    pub fn add_post(title: String, content: String, author: User) -> Post {
        let now: DateTime<Utc> = Utc::now();
        let now = now.format("%F %R");

        println!("UTC now is: {}", now);

        let post = Post {
            id: PostsService::get_all().len() as i32,
            date: now.to_string(),
            title,
            author,
            content,
        };

        PostsRepository::add_post(&post).unwrap();
        post
    }
}

pub struct CommentsService {}

impl CommentsService {
    pub fn get_all(post_id: usize) -> Vec<Comment> {
        CommentsRepository::get_all_from_post(post_id).unwrap()
    }

    pub fn create(post_id: usize, content: String, user: User) -> Comment {
        let now: DateTime<Utc> = Utc::now();
        let now = now.format("%F %R");

        let comment = Comment {
            id: 0,
            date: now.to_string(),
            author: user,
            content,
        };

        CommentsRepository::create(post_id, &comment).unwrap();
        comment
    }
}
