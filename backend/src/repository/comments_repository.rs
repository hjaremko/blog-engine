use crate::db::CONN;
use crate::model::{Comment, Rights, User};
use rusqlite::Result;
use std::str::FromStr;

pub struct CommentsRepository {}

impl CommentsRepository {
    pub fn init_tables() -> Result<()> {
        CONN.lock().unwrap().execute(
            "create table if not exists comments (
             id integer primary key,
             date date not null,
             author_id integer not null references users (id),
             content text not null,
             post_id integer not null references posts (id)
         )",
            [],
        )?;
        Ok(())
    }

    pub fn get_all_from_post(post_id: usize) -> Result<Vec<Comment>> {
        let conn = CONN.lock().unwrap();
        let mut stmt = conn.prepare(
            "select c.id, c.date, u.id, u.name, u.rights, c.content from comments c
                 join users u on c.author_id = u.id
                 where c.post_id = ?1",
        )?;

        let comment_iter = stmt.query_map([post_id], |row| {
            let r: String = row.get(4)?;
            Ok(Comment {
                id: row.get(0)?,
                date: row.get(1)?,
                author: User {
                    id: row.get(2)?,
                    name: row.get(3)?,
                    rights: Rights::from_str(&r).unwrap(),
                    password: "".to_string(),
                    login: "".to_string(),
                },
                content: row.get(5)?,
            })
        })?;

        Ok(comment_iter.map(|x| x.unwrap()).collect())
    }
}
