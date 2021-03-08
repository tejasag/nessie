use crate::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::http::{Cookie, CookieJar};
use rocket_contrib::databases::diesel::PgConnection;
use schema::users;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    // remember to hash
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_updated: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct InsertableUser<'a> {
    pub id: Uuid,
    pub username: &'a str,
    pub display_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub last_updated: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUserBody<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

fn current_time() -> NaiveDateTime {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let rightnow = NaiveDateTime::from_timestamp(since_the_epoch.as_secs() as i64, 0);
    rightnow
    //since_the_epoch
}

impl<'a> InsertableUser<'a> {
    pub fn new(
        username: &'a str,
        display_name: &'a str,
        email: &'a str,
        password: &'a str,
        conn: &PgConnection,
    ) -> User {
        let rightnow = current_time();
        let id = Uuid::new_v4();
        let user = Self {
            id,
            username,
            display_name,
            email,
            password,
            created_at: rightnow,
            last_updated: Option::from(rightnow),
        };

        diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
            .expect("Could not make a user")
    }

    pub fn update(id: Uuid, data_to_update: RequestUserBody, conn: &PgConnection) -> User {
        use schema::users::dsl::*;
        use schema::users::*;
        let rightnow = current_time();
        diesel::update(users.filter(id.eq(id)))
            .set((
                username.eq(data_to_update.username),
                display_name.eq(data_to_update.display_name),
                email.eq(data_to_update.email),
                password.eq(data_to_update.password),
                last_updated.eq(rightnow),
            ))
            .get_result::<User>(conn)
            .expect("Could not update the data")
    }
}
