pub mod auth_route;
pub mod user_route;

pub use auth_route::{login, register};
pub use user_route::{
    get_users, 
    get_user, 
    delete_user, 
    update_user, 
    profile, 
    update_profile
};