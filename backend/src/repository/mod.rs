use crate::db::CONN;
use crate::model::{Post, Rights, User, Comment, RegisterRequest};
use rusqlite::{params, Connection, Result};
use std::rc::Rc;
use std::str::FromStr;

pub struct UserRepository {}

impl UserRepository {
    pub fn init_tables() -> Result<()> {
        CONN.lock().unwrap().execute(
            "create table if not exists users (
             id integer primary key,
             login text not null,
             password text not null,
             name text not null,
             rights text not null,
             unique (login)
         )",
            [],
        )?;
        Ok(())
    }

    pub fn get_all() -> Result<Vec<User>> {
        let conn = CONN.lock().unwrap();
        let mut stmt = conn.prepare("select * from users")?;
        let post_iter = stmt.query_map([], |row| {
            let r: String = row.get(4)?;
            Ok(User {
                id: row.get(0)?,
                login: row.get(1)?,
                password: row.get(2)?,
                name: row.get(3)?,
                rights: Rights::from_str(&r).unwrap(),
            })
        })?;

        Ok(post_iter.map(|x| x.unwrap()).collect())
    }

    pub fn create(request: RegisterRequest) -> Result<()> {
        let conn = CONN.lock().unwrap();

        conn.execute(
            "insert into users (name, login, password, rights)
                values (?1, ?2, ?3, 'USER')",
            params![request.name, request.login, request.password],
        )?;

        Ok(())
    }
}

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
                 where c.post_id = ?1"
        )?;

        let comment_iter = stmt.query_map([post_id], |row| {
            let r:String = row.get(4)?;
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
            }
            )
        })?;

        Ok(comment_iter.map(|x| x.unwrap()).collect())
    }
}

