#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use models::*;

fn main() {
    routes::fuel(rocket::ignite()).launch();
}

#[cfg(test)]
mod tests;
