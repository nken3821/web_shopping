pub mod address_route;
pub mod auth_route;
pub mod user_route;

pub use address_route::{
    create_address, create_address_by_user, delete_address, delete_address_by_user, get_address,
    get_addresses, get_addresses_by_user, update_address, update_address_by_user,
};
pub use auth_route::{login, register};
pub use user_route::{
    create_user, delete_user, get_user, get_users, profile, update_profile, update_user,
};

use crate::repository::{AddressRepository, UserRepository};

pub const USER_REPO: UserRepository = UserRepository;
pub const ADDRESS_REPO: AddressRepository = AddressRepository;
