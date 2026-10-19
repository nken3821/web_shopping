use rocket::{routes};

use crate::routes::{login, register, get_users, get_user, delete_user, update_user, profile, update_profile};

mod config;
mod models;
mod schema;
mod repository;
mod internals;
mod interface;
mod service;
mod enums;
mod routes;
mod dtos;

#[rocket::main]
async fn main() {
    rocket::build()
        .manage(config::connect())
        .mount("/auth", routes![register, login])
        .mount("/", routes![get_users, get_user, delete_user, update_user, profile, update_profile])
        .launch().await.unwrap();
}


