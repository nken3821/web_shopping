use crate::dtos::user_request::{CreateUserRequest, UpdateUserRequest};
use crate::guards::auth_guard::{AdminUser, AuthenticatedUser};
use crate::interface::{Args, BaseService};
use crate::routes::USER_REPO;
use crate::service::user_service::{UserCreateArgs, UserUpdateArgs};
use crate::{config::Connection, dtos::user_response::UserResponse, service::UserService};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

// thiet ke lai ham nay
#[post("/", format = "json", data = "<data>")]
pub fn create_user(
    user: AdminUser,
    data: Json<CreateUserRequest>,
    connection: Connection,
) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.0.user_id),
        ..Default::default()
    };

    let user_ser = UserService::new(USER_REPO);

    let data = data.into_inner();

    let model_input = UserCreateArgs {
        email: data.email,
        password: data.password,
        phone: data.phone,
        fullname: data.fullname,
    };

    user_ser
        .create(model_input, args, &mut connection)
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

#[get("/")]
pub fn get_users(
    user: AdminUser,
    connection: Connection,
) -> Result<Json<Vec<UserResponse>>, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    let args = Args {
        actor_id: Some(user.0.user_id),
        ..Default::default()
    };

    user_ser
        .get_all(args, &mut connection)
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

#[get("/<id>")]
pub fn get_user(
    id: i32,
    user: AdminUser,
    connection: Connection,
) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.0.user_id),
        target_id: Some(id),
        ..Default::default()
    };

    let user_ser = UserService::new(USER_REPO);

    user_ser
        .get_by_id(args, &mut connection)
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

#[delete("/<id>")]
pub fn delete_user(id: i32, user: AdminUser, connection: Connection) -> Result<Status, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.0.user_id),
        target_id: Some(id),
        ..Default::default()
    };

    let user_ser = UserService::new(USER_REPO);

    let args = UserDeleteArgs {
        model_id: None,
        list_id: None,
    };

    user_ser
        .delete(args, &mut connection)
        .map(|_| Status::NoContent)
        .map_err(|error| error.status())
}

#[put("/<id>", format = "json", data = "<data>")]
pub fn update_user(
    id: i32,
    user: AdminUser,
    connection: Connection,
    data: Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    let args = Args {
        actor_id: Some(user.0.user_id),
        target_id: Some(id),
        ..Default::default()
    };

    let data = data.into_inner();

    let model_input = UserUpdateArgs {
        email: data.email,
        phone: data.phone,
        fullname: data.fullname,
    };

    user_ser
        .update(model_input, args, &mut connection)
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

    let args = Args {
        actor_id: Some(user.user_id),
        target_id: Some(user.user_id),
        ..Default::default()
    };

    let user_ser = UserService::new(USER_REPO);

    user_ser
        .get_by_id(args, &mut connection)
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

    let args = Args {
        actor_id: Some(user.user_id),
        target_id: Some(user.user_id),
        ..Default::default()
    };

    let user_ser = UserService::new(USER_REPO);

    let data = data.into_inner();

    let model_input = UserUpdateArgs {
        email: data.email,
        phone: data.phone,
        fullname: data.fullname,
    };

    user_ser
        .update(model_input, args, &mut connection)
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
