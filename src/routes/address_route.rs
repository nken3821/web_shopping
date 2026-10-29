use crate::dtos::address_request::{CreateAddressRequest, UpdateAddressRequest};
use crate::dtos::address_response::AddressResponse;
use crate::guards::auth_guard::{AdminUser, AuthenticatedUser};
use crate::interface::{Args, BaseService};
use crate::routes::ADDRESS_REPO;
use crate::service::address_service::{AddressCreateArgs, AddressUpdateArgs};
use crate::{config::Connection, service::AddressService};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

#[post("/<id>/addresses", format = "json", data = "<data>")]
pub fn create_address(
    id: i32,
    user: AdminUser,
    data: Json<CreateAddressRequest>,
    connection: Connection,
) -> Result<Json<AddressResponse>, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.0.user_id),
        middle_id: Some(id),
        ..Default::default()
    };

    let address_ser = AddressService::new(ADDRESS_REPO);

    let data = data.into_inner();

    let model_input = AddressCreateArgs {
        user_id: id,
        receiver_name: data.receiver_name,
        phone: data.phone,
        province: data.province,
        district: data.district,
        ward: data.ward,
        street: data.street,
    };

    address_ser
        .create(model_input, args, &mut connection)
        .map(|address| {
            Json(AddressResponse {
                id: address.id,
                receiver_name: address.receiver_name,
                phone: address.phone,
                province: address.province,
                district: address.district,
                ward: address.ward,
                street: address.street,
                is_default: address.is_default,
                created_at: address.created_at.to_string(),
                updated_at: address.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}

#[get("/<id>/addresses")]
pub fn get_addresses(
    id: i32,
    user: AdminUser,
    connection: Connection,
) -> Result<Json<Vec<AddressResponse>>, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.0.user_id),
        middle_id: Some(id),
        ..Default::default()
    };

    let address_ser = AddressService::new(ADDRESS_REPO);

    address_ser
        .get_all(args, &mut connection)
        .map(|addresses| {
            Json(
                addresses
                    .into_iter()
                    .map(|address| AddressResponse {
                        id: address.id,
                        receiver_name: address.receiver_name,
                        phone: address.phone,
                        province: address.province,
                        district: address.district,
                        ward: address.ward,
                        street: address.street,
                        is_default: address.is_default,
                        created_at: address.created_at.to_string(),
                        updated_at: address.updated_at.to_string(),
                    })
                    .collect(),
            )
        })
        .map_err(|error| error.status())
}

#[get("/<id>/addresses/<target_id>")]
pub fn get_address(
    id: i32,
    target_id: i32,
    user: AdminUser,
    connection: Connection,
) -> Result<Json<AddressResponse>, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.0.user_id),
        middle_id: Some(id),
        target_id: Some(target_id),
        ..Default::default()
    };

    let address_ser = AddressService::new(ADDRESS_REPO);

    address_ser
        .get_by_id(args, &mut connection)
        .map(|address| {
            Json(AddressResponse {
                id: address.id,
                receiver_name: address.receiver_name,
                phone: address.phone,
                province: address.province,
                district: address.district,
                ward: address.ward,
                street: address.street,
                is_default: address.is_default,
                created_at: address.created_at.to_string(),
                updated_at: address.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}

#[delete("/<id>/addresses/<target_id>")]
pub fn delete_address(
    id: i32,
    target_id: i32,
    user: AdminUser,
    connection: Connection,
) -> Result<Status, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.0.user_id),
        middle_id: Some(id),
        target_id: Some(target_id),
        ..Default::default()
    };

    let address_ser = AddressService::new(ADDRESS_REPO);

    address_ser
        .delete(args, &mut connection)
        .map(|_| Status::NoContent)
        .map_err(|error| error.status())
}

#[put("/<id>/addresses/<target_id>", format = "json", data = "<data>")]
pub fn update_address(
    id: i32,
    target_id: i32,
    user: AdminUser,
    connection: Connection,
    data: Json<UpdateAddressRequest>,
) -> Result<Json<AddressResponse>, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.0.user_id),
        middle_id: Some(id),
        target_id: Some(target_id),
        ..Default::default()
    };

    let address_ser = AddressService::new(ADDRESS_REPO);

    let data = data.into_inner();

    let model_input = AddressUpdateArgs {
        receiver_name: data.receiver_name,
        phone: data.phone,
        province: data.province,
        district: data.district,
        ward: data.ward,
        street: data.street,
        is_default: data.is_default,
    };

    address_ser
        .update(model_input, args, &mut connection)
        .map(|address| {
            Json(AddressResponse {
                id: address.id,
                receiver_name: address.receiver_name,
                phone: address.phone,
                province: address.province,
                district: address.district,
                ward: address.ward,
                street: address.street,
                is_default: address.is_default,
                created_at: address.created_at.to_string(),
                updated_at: address.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}

#[get("/addresses")]
pub fn get_addresses_by_user(
    user: AuthenticatedUser,
    connection: Connection,
) -> Result<Json<Vec<AddressResponse>>, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.user_id),
        middle_id: Some(user.user_id),
        ..Default::default()
    };

    let address_ser = AddressService::new(ADDRESS_REPO);

    address_ser
        .get_all(args, &mut connection)
        .map(|addresses| {
            Json(
                addresses
                    .into_iter()
                    .map(|address| AddressResponse {
                        id: address.id,
                        receiver_name: address.receiver_name,
                        phone: address.phone,
                        province: address.province,
                        district: address.district,
                        ward: address.ward,
                        street: address.street,
                        is_default: address.is_default,
                        created_at: address.created_at.to_string(),
                        updated_at: address.updated_at.to_string(),
                    })
                    .collect(),
            )
        })
        .map_err(|error| error.status())
}

#[put("/addresses/<target_id>", format = "json", data = "<data>")]
pub fn update_address_by_user(
    target_id: i32,
    user: AuthenticatedUser,
    connection: Connection,
    data: Json<UpdateAddressRequest>,
) -> Result<Json<AddressResponse>, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.user_id),
        middle_id: Some(user.user_id),
        target_id: Some(target_id),
        ..Default::default()
    };

    let address_ser = AddressService::new(ADDRESS_REPO);

    let data = data.into_inner();

    let model_input = AddressUpdateArgs {
        receiver_name: data.receiver_name,
        phone: data.phone,
        province: data.province,
        district: data.district,
        ward: data.ward,
        street: data.street,
        is_default: data.is_default,
    };

    address_ser
        .update(model_input, args, &mut connection)
        .map(|address| {
            Json(AddressResponse {
                id: address.id,
                receiver_name: address.receiver_name,
                phone: address.phone,
                province: address.province,
                district: address.district,
                ward: address.ward,
                street: address.street,
                is_default: address.is_default,
                created_at: address.created_at.to_string(),
                updated_at: address.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}

#[delete("/addresses/<target_id>")]
pub fn delete_address_by_user(
    target_id: i32,
    user: AuthenticatedUser,
    connection: Connection,
) -> Result<Status, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.user_id),
        middle_id: Some(user.user_id),
        target_id: Some(target_id),
        ..Default::default()
    };

    let address_ser = AddressService::new(ADDRESS_REPO);

    address_ser
        .delete(args, &mut connection)
        .map(|_| Status::NoContent)
        .map_err(|error| error.status())
}

#[post("/addresses", format = "json", data = "<data>")]
pub fn create_address_by_user(
    data: Json<CreateAddressRequest>,
    user: AuthenticatedUser,
    connection: Connection,
) -> Result<Json<AddressResponse>, Status> {
    let mut connection = connection.0;

    let args = Args {
        actor_id: Some(user.user_id),
        middle_id: Some(user.user_id),
        ..Default::default()
    };

    let address_ser = AddressService::new(ADDRESS_REPO);

    let data = data.into_inner();

    let model_input = AddressCreateArgs {
        user_id: user.user_id,
        receiver_name: data.receiver_name,
        phone: data.phone,
        province: data.province,
        district: data.district,
        ward: data.ward,
        street: data.street,
    };
    address_ser
        .create(model_input, args, &mut connection)
        .map(|address| {
            Json(AddressResponse {
                id: address.id,
                receiver_name: address.receiver_name,
                phone: address.phone,
                province: address.province,
                district: address.district,
                ward: address.ward,
                street: address.street,
                is_default: address.is_default,
                created_at: address.created_at.to_string(),
                updated_at: address.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}
