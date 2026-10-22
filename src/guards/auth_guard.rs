use crate::config::Connection;
use crate::interface::BaseRepository;
use crate::service::token_service::{TokenService};
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use std::env;

use crate::routes::USER_REPO;
use crate::enums::role::UserRole;

pub struct AuthenticatedUser {
    pub user_id: i32,
    pub role: UserRole,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = request.headers().get_one("Authorization");

        let authorization = match authorization {
            Some(value) => value,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let token = match authorization.strip_prefix("Bearer ") {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let secret = match env::var("JWT_SECRET") {
            Ok(secret) => secret,
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };

       let data = match TokenService::verify_token(token, secret) {
            Ok(data) => data,
            Err(_) => {
                return Outcome::Error((Status::Unauthorized, ()));
            }
        };

        let user_id = data.claims.sub;

        let mut connection = match request.guard::<Connection>().await {
            Outcome::Success(connection) => connection,
            _ => {
                return Outcome::Error((Status::InternalServerError, ()));
            }
        };

        let user = match USER_REPO.find_by_id(user_id, &mut connection.0) {
            Ok(user) => user,
            Err(diesel::result::Error::NotFound) => {
                return Outcome::Error((Status::Unauthorized, ()));
            }
            Err(_) => {
                return Outcome::Error((Status::InternalServerError, ()));
            }
        };

        let role = match UserRole::try_from(user.role.as_str()) {
            Ok(role) => role,
            Err(_) => {
                return Outcome::Error((Status::InternalServerError, ()));
            }
        };

        Outcome::Success(AuthenticatedUser { user_id, role })
    }
}

pub struct AdminUser(pub AuthenticatedUser);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match AuthenticatedUser::from_request(req).await {
            Outcome::Success(user) if user.role == UserRole::Admin => Outcome::Success(AdminUser(user)),
            Outcome::Success(_) => Outcome::Error((Status::Forbidden, ())),
            Outcome::Error(error) => Outcome::Error(error),
            Outcome::Forward(status) => Outcome::Forward(status),
        }
    }
}
