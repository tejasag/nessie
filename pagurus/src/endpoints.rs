use crate::models::{InsertableUser, RequestUserBody, User};
use rocket::request::Request;
use rocket::Rocket;
use rocket_contrib::{databases::diesel::PgConnection, json::Json};
use serde::{Deserialize, Serialize};

#[post("/create-user", format = "json", data = "<input>")]
pub fn create(input: Json<RequestUserBody>, db_conn: PostgresDB) -> Json<User> {
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
pub fn index(_db_conn: PostgresDB) -> &'static str {
    "Welcome to pagurus!"
}

#[catch(503)]
pub fn unavailable(_req: &Request) -> &'static str {
    "Service is not available"
}

#[database("PostgresDB")]
pub struct PostgresDB(PgConnection);

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket
        .attach(PostgresDB::fairing())
        .mount("/", routes![index, create, edit])
        .register(catchers![unavailable])
}
