pub mod auth_route;
pub mod user_route;

pub use auth_route::{login, register};
pub use user_route::{delete_user, get_user, get_users, profile, update_profile, update_user};

use crate::repository::UserRepository;

pub const USER_REPO: UserRepository = UserRepository;