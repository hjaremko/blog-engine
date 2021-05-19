use rusqlite::{Connection, Result, params};
use std::rc::Rc;
use crate::db::CONN;
use crate::model::{Post, User, Rights};

pub struct UserRepository {}

impl UserRepository {
    pub fn init_tables() -> Result<()> {
        CONN.lock().unwrap().execute(
            "create table if not exists users (
             id integer primary key,
             name text not null,
             rights text not null
         )",
            [],
        )?;
        Ok(())
    }

    pub fn get_all() -> Result<Vec<User>> {
        let conn = CONN.lock().unwrap();
        let mut stmt = conn.prepare("select * from users")?;
        let post_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                // rights: row.get(2)?,
                rights: Rights::Common,
            })
        })?;

        Ok(post_iter.map(|x| x.unwrap()).collect())
    }
}

pub struct PostsRepository {}

impl PostsRepository {
    pub fn init_tables() -> Result<()> {
        CONN.lock().unwrap().execute(
            "create table if not exists posts (
             id integer primary key,
             date text not null,
             title text not null,
             author text not null,
             content text not null
         )",
            [],
        )?;
        Ok(())
    }

    pub fn get_all() -> Result<Vec<Post>> {
        let conn = CONN.lock().unwrap();
        let mut stmt = conn.prepare("select * from posts join users on posts.author")?;
        // let mut stmt = conn.prepare("select * from posts")?;
        let post_iter = stmt.query_map([], |row| {
            Ok(Post {
                id: row.get(0)?,
                date: row.get(1)?,
                title: row.get(2)?,
                // author: row.get(3)?,
                author: User {
                    id: row.get(5)?,
                    name: row.get(6)?,
                    rights: Rights::Administrator,
                    // rights: row.get(7)?,
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
