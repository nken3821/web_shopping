use crate::dtos::user_request::UpdateUserRequest;
use crate::guards::auth_guard::{AdminUser, AuthenticatedUser};
use crate::interface::BaseService;
use crate::routes::USER_REPO;
use crate::service::user_service::{UserDeleteArgs, UserUpdateArgs};
use crate::{config::Connection, dtos::user_response::UserResponse, service::UserService};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, put};

#[get("/users")]
pub fn get_users(_user: AdminUser, connection: Connection) -> Result<Json<Vec<UserResponse>>, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    
    user_ser
        .get_all(None, &mut connection)
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
pub fn get_user(id: i32, _user: AdminUser, connection: Connection) -> Result<Json<UserResponse>, Status> {
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
pub fn delete_user(id: i32, _user: AdminUser, connection: Connection) -> Result<Status, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    let args = UserDeleteArgs {
        model_id: None,
        list_id: None,
    };

    user_ser
        .delete(&mut connection, id, args)
        .map(|_| Status::NoContent)
        .map_err(|error| error.status())
}

#[put("/users/<id>", format = "json", data = "<data>")]
pub fn update_user(
    id: i32,
    _user: AdminUser,
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

