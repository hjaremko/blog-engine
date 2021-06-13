use crate::db::CONN;
use crate::model::{RegisterRequest, Rights, User};
use rusqlite::{params, Result};
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
