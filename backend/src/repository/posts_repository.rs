use crate::db::CONN;
use crate::model::{Post, Rights, User};
use rusqlite::{params, Result};
use std::str::FromStr;

pub struct PostsRepository {}

impl PostsRepository {
    pub fn init_tables() -> Result<()> {
        CONN.lock().unwrap().execute(
            "create table if not exists posts (
             id integer primary key,
             date date not null,
             title text not null,
             author integer not null,
             content text not null
         )",
            [],
        )?;
        Ok(())
    }

    pub fn get_all() -> Result<Vec<Post>> {
        let conn = CONN.lock().unwrap();
        let mut stmt = conn.prepare(
            "select * from posts
                 join users u on author = u.id
                 order by date desc",
        )?;
        let post_iter = stmt.query_map([], |row| {
            let rights: String = row.get(9)?;
            Ok(Post {
                id: row.get(0)?,
                date: row.get(1)?,
                title: row.get(2)?,
                // author: row.get(3)?,
                author: User {
                    id: row.get(5)?,
                    name: row.get(8)?,
                    rights: Rights::from_str(&rights).unwrap(),
                    password: "".to_string(),
                    login: "".to_string(),
                },
                content: row.get(4)?,
            })
        })?;

        Ok(post_iter.map(|x| x.unwrap()).collect())
    }

    pub fn add_post(post: &Post) -> Result<()> {
        let conn = CONN.lock().unwrap();

        conn.execute(
            "insert into posts (date, title, author, content) values (?1, ?2, ?3, ?4)",
            params![post.date, post.title, post.author.id, post.content],
        )?;

        Ok(())
    }
}
