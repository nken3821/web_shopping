use rocket::routes;

use crate::routes::{
    delete_user, get_user, get_users, login, profile, register, update_profile, update_user,
};

mod config;
mod dtos;
mod enums;
mod interface;
mod internals;
mod models;
mod repository;
mod routes;
mod schema;
mod service;

#[rocket::main]
async fn main() {
    rocket::build()
        .manage(config::connect())
        .mount("/auth", routes![register, login])
        .mount(
            "/",
            routes![
                get_users,
                get_user,
                delete_user,
                update_user,
                profile,
                update_profile
            ],
        )
        .launch()
        .await
        .unwrap();
}
