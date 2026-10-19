use crate::dtos::user_request::UpdateUserRequest;
use crate::interface::BaseService;
use crate::repository::UserRepository;
use crate::service::token_service::Claims;
use crate::service::user_service::UserUpdateArgs;
use crate::{config::Connection, dtos::user_response::UserResponse, service::UserService};
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::{delete, get, put};
use std::env;

const USER_REPO: UserRepository = UserRepository;

pub struct AuthenticatedUser {
    pub user_id: i32,
}

#[get("/users")]
pub fn get_users(connection: Connection) -> Result<Json<Vec<UserResponse>>, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    user_ser
        .get_all(&mut connection)
        .map(|users| {
            Json(
                users
                    .into_iter()
                    .map(|user| UserResponse {
                        id: user.id,
                        username: user.user_name,
                        email: user.email,
                        phone: user.phone,
                        fullname: user.full_name,
                        created_at: user.created_at.to_string(),
                        updated_at: user.updated_at.to_string(),
                    })
                    .collect(),
            )
        })
        .map_err(|error| error.status())
}

#[get("/users/<id>")]
pub fn get_user(id: i32, connection: Connection) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    user_ser
        .get_by_id(&mut connection, id)
        .map(|user| {
            Json(UserResponse {
                id: user.id,
                username: user.user_name,
                email: user.email,
                phone: user.phone,
                fullname: user.full_name,
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}

#[delete("/users/<id>")]
pub fn delete_user(id: i32, connection: Connection) -> Result<Status, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    user_ser
        .delete(&mut connection, id)
        .map(|_| Status::NoContent)
        .map_err(|error| error.status())
}

#[put("/users/<id>", format = "json", data = "<data>")]
pub fn update_user(
    id: i32,
    connection: Connection,
    data: Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    let data = data.into_inner();

    let args = UserUpdateArgs {
        email: data.email,
        phone: data.phone,
        fullname: data.fullname,
    };

    user_ser
        .update(&mut connection, id, args)
        .map(|user| {
            Json(UserResponse {
                id: user.id,
                username: user.user_name,
                email: user.email,
                phone: user.phone,
                fullname: user.full_name,
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}

#[get("/profile")]
pub fn profile(
    user: AuthenticatedUser,
    connection: Connection,
) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    user_ser
        .get_by_id(&mut connection, user.user_id)
        .map(|user| {
            Json(UserResponse {
                id: user.id,
                username: user.user_name,
                email: user.email,
                phone: user.phone,
                fullname: user.full_name,
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}

#[put("/profile", format = "json", data = "<data>")]
pub fn update_profile(
    user: AuthenticatedUser,
    connection: Connection,
    data: Json<UpdateUserRequest>,
    // phan them updateUserRequest cua admin
) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    let data = data.into_inner();

    let args = UserUpdateArgs {
        email: data.email,
        phone: data.phone,
        fullname: data.fullname,
    };

    user_ser
        .update(&mut connection, user.user_id, args)
        .map(|user| {
            Json(UserResponse {
                id: user.id,
                username: user.user_name,
                email: user.email,
                phone: user.phone,
                fullname: user.full_name,
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
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

        let validation = jsonwebtoken::Validation::default();

        let token_data = match jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        ) {
            Ok(data) => data,
            Err(_) => return Outcome::Error((Status::Unauthorized, ())),
        };

        Outcome::Success(AuthenticatedUser {
            user_id: token_data.claims.sub,
        })
    }
}
