use crate::*;
use rocket::http::ContentType;
use rocket::local::Client;

#[test]
fn test_index() {
    let rocket = endpoints::fuel(rocket::ignite());
    let client = Client::new(rocket).expect("invalid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Welcome to pagurus!".into()))
}

#[test]
fn test_user_create() {
    let rocket = endpoints::fuel(rocket::ignite());
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
    let user: User = serde_json::from_str(&response_body.as_str()).expect("Valid user response.");
    assert_eq!(user.username, String::from("tejasagarwal"));
    assert_eq!(user.display_name, String::from("Tejas Agarwal"));
    assert_eq!(user.email, String::from("tejas@agarw.al"));
    assert_eq!(user.password, String::from("secretPassword"));
}

#[test]
fn test_user_edit() {
    let rocket = endpoints::fuel(rocket::ignite());
    let client = Client::new(rocket).expect("invalid rocket instance");
    let mut response = client
        .post("/edit-user")
        .header(ContentType::JSON)
        .body(
            r##"{
                "username": "tejasagarwal",
                    "new_username": "tejasag",
                    "display_name": "Tejas Agarwal",
                    "email": "me@tejasagarwal.tech",
                    "password": "secretPassword"
            }"##,
        )
        .dispatch();
    let response_body = response.body_string().expect("Response body");
    let user: User = serde_json::from_str(&response_body.as_str()).expect("Invalid user response.");
    assert_eq!(user.username, String::from("tejasag"));
    assert_eq!(user.display_name, String::from("Tejas Agarwal"));
    assert_eq!(user.email, String::from("me@tejasagarwal.tech"));
    assert_eq!(user.password, String::from("secretPassword"));
}
