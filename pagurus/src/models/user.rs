use crate::schema::users;
use diesel::{prelude::*, PgConnection, *};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub last_updated: Option<String>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct InsertableUser<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: &'a str,
    pub last_updated: &'a str,
}

impl User {
    pub fn new<'a>(
        username: &'a str,
        display_name: &'a str,
        email: &'a str,
        password: &'a str,
        created_at: &'a str,
        conn: &PgConnection,
    ) -> User {
        let u = InsertableUser {
            username,
            display_name,
            email,
            password,
            created_at,
            last_updated: created_at,
        };

        diesel::insert_into(users::table)
            .values(&u)
            .get_result::<User>(conn)
            .expect("Error saving user")
    }

    pub fn delete(user_id: &str, conn: &PgConnection) -> usize {
        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(user_id.parse::<i32>().unwrap())))
            .execute(conn)
            .expect("Error deleting user")
    }

    pub fn edit<'a>(
        user_id: &'a str,
        username: &'a str,
        display_name: &'a str,
        email: &'a str,
        password: &'a str,
        current_time: &'a str,
        conn: &PgConnection,
    ) -> User {
        diesel::update(users::table.filter(users::id.eq(user_id.parse::<i32>().unwrap())))
            .set((
                users::username.eq(username),
                users::display_name.eq(display_name),
                users::email.eq(email),
                users::password.eq(password),
                users::last_updated.eq(current_time),
            ))
            .get_result(conn)
            .expect("Could not update the user")
    }

    pub fn get_user(user_id: &str, conn: &PgConnection) -> Vec<User> {
        use crate::schema::users::dsl::*;

        users
            .filter(id.eq(user_id.parse::<i32>().unwrap()))
            .limit(1)
            .load::<User>(conn)
            .expect("Error fetching user")
    }
}
