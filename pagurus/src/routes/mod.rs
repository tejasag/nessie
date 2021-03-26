use diesel::PgConnection;
use rocket::Rocket;

pub mod user;
use crate::user::static_rocket_route_info_for_user_create;

#[database("postgresdb")]
pub struct Database(PgConnection);

pub fn fuel(r: Rocket) -> Rocket {
    r.attach(Database::fairing())
        .mount("/", routes![user_create])
}
