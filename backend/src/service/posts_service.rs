use crate::model::{Post, User};
use crate::repository::PostsRepository;
use chrono::{DateTime, Utc};

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
