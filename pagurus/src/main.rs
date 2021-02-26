#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod models;
mod schema;

use models::*;

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
mod tests;
