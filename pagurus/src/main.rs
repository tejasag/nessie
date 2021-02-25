#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod schema;
mod utils;

use diesel::prelude::*;
use rocket::request::Request;
use rocket_contrib::{databases::diesel::PgConnection, json::Json};
use serde::{Deserialize, Serialize};

#[post("/create-user", format = "json", data = "<input>")]
fn create(input: Json<InsertableUser>, db_conn: PostgresDB) -> Json<User> {
    Json(InsertableUser::new(
        input.username,
        input.display_name,
        input.email,
        input.password,
        input.created_at,
        input.last_updated,
        &db_conn.0,
    ))
}

#[get("/")]
fn index(_db_conn: PostgresDB) -> &'static str {
    "Welcome to pagurus!"
}

#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service is not available"
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub email: String,
    // remember to hash
    pub password: String,
    pub created_at: u64,
    pub last_updated: Option<u64>,
}

use rocket::response::content::JavaScript;
use schema::users;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct InsertableUser<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: &'a u64,
    pub last_updated: Option<&'a u64>,
}

impl<'a> InsertableUser<'a> {
    fn new(
        username: &'a str,
        display_name: &'a str,
        email: &'a str,
        password: &'a str,
        created_at: u64,
        last_updated: Option<u64>,
        conn: &PgConnection,
    ) -> User {
        use schema::users;

        let user = Self {
            username,
            display_name,
            email,
            password,
            created_at,
            last_updated,
        };

        diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
            .expect("Could not make a user")
    }
}

#[database("PostgresDB")]
struct PostgresDB(PgConnection);

fn main() {
    rocket::ignite()
        .attach(PostgresDB::fairing())
        .register(catchers![serivce_not_available])
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
            .register(catchers![service_not_available])
            .mount("/", routes![index, create]);
        let client = Client::new(rocket).expect("invalid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.body_string(), Some("Welcome to pagurus!").into())
    }
}
