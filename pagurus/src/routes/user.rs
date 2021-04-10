use crate::models::user::User;
use crate::Database;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserCreateJson {
    username: String,
    display_name: String,
    id: String,
    email: String,
    // remember to hash
    password: String,
    created_at: String,
    last_updated: Option<String>,
}

#[post("/user/create", format = "json", data = "<input>")]
pub fn user_create(input: Json<UserCreateJson>, conn: Database) -> Json<User> {
    Json(User::new(
        &input.username,
        &input.display_name,
        &input.email,
        &input.password,
        &input.created_at,
        &conn.0,
    ))
}

#[derive(Serialize, Deserialize)]
pub struct UserDeleteJson {
    user_id: String,
}

#[post("/user/delete", format = "json", data = "<input>")]
pub fn user_delete(input: Json<UserDeleteJson>, conn: Database) -> Json<usize> {
    Json(User::delete(&input.user_id, &conn.0))
}
