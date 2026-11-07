use rocket::routes;

use crate::routes::{
    create_user, delete_more_user_by_admin, delete_user, get_user, get_users, login, profile,
    register, update_profile, update_user,
};

use crate::routes::{
    create_address, create_address_by_user, delete_address, delete_address_by_user,
    delete_more_address_by_admin_user, delete_more_address_by_user, get_address, get_addresses,
    get_addresses_by_user, update_address, update_address_by_user,
};

use crate::routes::get_activities;

mod config;
mod dtos;
mod enums;
mod guards;
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
            "/users",
            routes![
                get_users,
                get_user,
                delete_user,
                update_user,
                get_addresses,
                get_address,
                update_address,
                delete_address,
                create_user,
                create_address,
                delete_more_address_by_admin_user,
                delete_more_user_by_admin,
            ],
        )
        .mount("/historys", routes![get_activities])
        .mount(
            "/",
            routes![
                profile,
                update_profile,
                get_addresses_by_user,
                update_address_by_user,
                delete_address_by_user,
                create_address_by_user,
                delete_more_address_by_user,
            ],
        )
        .launch()
        .await
        .unwrap();
}
