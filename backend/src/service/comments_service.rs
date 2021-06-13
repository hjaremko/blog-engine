use crate::model::{Comment, User};
use crate::repository::CommentsRepository;
use chrono::{DateTime, Utc};

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
