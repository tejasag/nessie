#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod endpoints;
mod models;
mod schema;

use models::*;

fn main() {
    endpoints::fuel(rocket::ignite()).launch();
}

#[cfg(test)]
mod tests;
