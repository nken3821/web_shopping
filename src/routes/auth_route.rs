use crate::{
    config::Connection, dtos::{
        auth_request::{LoginRequest, RegisterRequest},
        auth_response::LoginResponse,
        user_response::UserResponse,
    }, enums::{role::UserRole}, interface::BaseService, routes::USER_REPO, service::{
        UserService, login_service::LoginService, token_service::TokenService,
        user_service::UserCreateArgs,
    },
};
use rocket::serde::json::Json;
use rocket::{http::Status, post};

// const USER_REPO: UserRepository = UserRepository;

#[post("/login", format = "json", data = "<data>")]
pub fn login(
    data: Json<LoginRequest>,
    connection: Connection,
) -> Result<Json<LoginResponse>, Status> {
    let mut connection = connection.0;

    let login_ser = LoginService::new(USER_REPO);

    login_ser
        .login(&mut connection, &data.password, &data.email)
        .map_err(|error| error.status())
        .and_then(|user| {
            let role = UserRole::try_from(user.role.as_str()).map_err(|_| Status::BadRequest)?;
            let token =
                TokenService::generate_token(user.id, role).map_err(|_| Status::InternalServerError)?;

            Ok(Json(LoginResponse { token }))
        })
}

#[post("/register", format = "json", data = "<data>")]
pub fn register(
    data: Json<RegisterRequest>,
    connection: Connection,
) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    let user_ser = UserService::new(USER_REPO);

    let data = data.into_inner();

    let args = UserCreateArgs {
        email: data.email,
        password: data.password,
        phone: data.phone,
        fullname: data.fullname,
    };

    user_ser
        .create(&mut connection, args)
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
