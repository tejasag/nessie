#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod schema;

use diesel::prelude::*;
use rocket::request::Request;
use rocket_contrib::{databases::diesel::PgConnection, json::Json};
use serde::{Deserialize, Serialize};

#[post("/create-user", format = "json", data = "<input>")]
fn create(input: Json<RequestUserBody>, db_conn: PostgresDB) -> Json<User> {
    Json(InsertableUser::new(
        input.username,
        input.display_name,
        input.email,
        input.password,
        &db_conn.0,
    ))
}

#[derive(Serialize, Deserialize)]
struct EditUserData<'a> {
    username: &'a str,
    new_username: &'a str,
    display_name: &'a str,
    email: &'a str,
    password: &'a str,
}

#[post("/edit-user", format = "json", data = "<input>")]
fn edit(input: Json<EditUserData>, db_conn: PostgresDB) -> Json<User> {
    let data = RequestUserBody {
        username: input.new_username,
        display_name: input.display_name,
        email: input.email,
        password: input.password,
    };
    Json(InsertableUser::update(input.username, data, &db_conn.0))
}

#[get("/")]
fn index(_db_conn: PostgresDB) -> &'static str {
    "Welcome to pagurus!"
}

#[catch(503)]
fn unavailable(_req: &Request) -> &'static str {
    "Service is not available"
}

#[derive(Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub email: String,
    // remember to hash
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_updated: Option<chrono::NaiveDateTime>,
}

use chrono::NaiveDateTime;
use schema::users;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct InsertableUser<'a> {
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
    use chrono::{DateTime, Utc};
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let rightnow = NaiveDateTime::from_timestamp(since_the_epoch.as_secs() as i64, 0);
    rightnow
}

impl<'a> InsertableUser<'a> {
    fn new(
        username: &'a str,
        display_name: &'a str,
        email: &'a str,
        password: &'a str,
        conn: &PgConnection,
    ) -> User {
        let rightnow = current_time();
        let user = Self {
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

    fn update(
        username_input: &'a str,
        data_to_update: RequestUserBody,
        conn: &PgConnection,
    ) -> User {
        use schema::users::dsl::*;
        use schema::users::*;
        let rightnow = current_time();
        diesel::update(users.filter(username.like(username_input)))
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

#[database("PostgresDB")]
struct PostgresDB(PgConnection);

fn main() {
    rocket::ignite()
        .attach(PostgresDB::fairing())
        .register(catchers![unavailable])
        .mount("/", routes![index, create])
        .launch();
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use rocket_contrib::json::Json;

    #[test]
    fn test_index() {
        let rocket = rocket::ignite()
            .attach(PostgresDB::fairing())
            .register(catchers![unavailable])
            .mount("/", routes![index, create]);
        let client = Client::new(rocket).expect("invalid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.body_string(), Some("Welcome to pagurus!".into()))
    }

    #[test]
    fn test_user_create() {
        let rocket = rocket::ignite()
            .attach(PostgresDB::fairing())
            .register(catchers![unavailable])
            .mount("/", routes![index, create]);
        let client = Client::new(rocket).expect("invalid rocket instance");
        let mut response = client
            .post("/create-user")
            .header(ContentType::JSON)
            .body(
                r##"{
                "username": "tejasagarwal",
                "display_name": "Tejas Agarwal",
                "email": "tejas@agarw.al",
                "password": "secretPassword"
            }"##,
            )
            .dispatch();
        let response_body = response.body_string().expect("Response body");
        let user: User =
            serde_json::from_str(&response_body.as_str()).expect("Valid user response.");
        assert_eq!(user.username, String::from("tejasagarwal"));
        assert_eq!(user.display_name, String::from("Tejas Agarwal"));
        assert_eq!(user.email, String::from("tejas@agarw.al"));
        assert_eq!(user.password, String::from("secretPassword"));
    }
    #[test]
    fn test_user_edit() {
        let rocket = rocket::ignite()
            .attach(PostgresDB::fairing())
            .register(catchers![unavailable])
            .mount("/", routes![index, create, edit]);
        let client = Client::new(rocket).expect("invalid rocket instance");
        let mut response = client
            .post("/edit-user")
            .header(ContentType::JSON)
            .body(
                r##"{
                "username": "tejasagarwal",
                    "new_username": "tejasagarwal",
                    "display_name": "Tejas Agarwal",
                    "email": "me@tejasagarwal.tech",
                    "password": "secretPassword"
            }"##,
            )
            .dispatch();
        let response_body = response.body_string().expect("Response body");
        let user: User =
            serde_json::from_str(&response_body.as_str()).expect("Invalid user response.");
        assert_eq!(user.username, String::from("tejasagarwal"));
        assert_eq!(user.display_name, String::from("Tejas Agarwal"));
        assert_eq!(user.email, String::from("me@tejasagarwal.tech"));
        assert_eq!(user.password, String::from("secretPassword"));
    }
}
