use diesel::MysqlConnection;

use crate::{
    enums::AppError,
    interface::{Args, BaseRepository, BaseService},
    internals::AddressInternal,
    models::{NewAddress, UpdateAdress},
    repository::AddressRepository,
};

pub struct AddressService {
    address_repo: AddressRepository,
}

pub struct AddressCreateArgs {
    pub user_id: i32,
    pub receiver_name: String,
    pub phone: String,
    pub province: String,
    pub district: Option<String>,
    pub ward: String,
    pub street: String,
}

pub struct AddressUpdateArgs {
    pub receiver_name: Option<String>,
    pub phone: Option<String>,
    pub province: Option<String>,
    pub district: Option<String>,
    pub ward: Option<String>,
    pub street: Option<String>,
    pub is_default: Option<bool>,
}

impl BaseService for AddressService {
    type InsertModelInput = AddressCreateArgs;
    type UpdateModelInput = AddressUpdateArgs;
    type AgrsInput = Args;
    type OutputModel = AddressInternal;
    type OutputError = AppError;

    fn get_all(
        &self,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<Vec<AddressInternal>, AppError> {
        let actor_id = self.actor_work(args.actor_id)?;
        let user_id = args.middle_id.ok_or(AppError::ValidationError)?;
        self.address_repo
            .find_all(user_id, connection)
            .map(|result| {
                result
                    .addresses
                    .into_iter()
                    .map(|address| AddressInternal {
                        id: address.id,
                        receiver_name: address.receiver_name,
                        phone: address.phone,
                        province: address.province,
                        district: address.district,
                        ward: address.ward,
                        street: address.street,
                        is_default: address.is_default,
                        created_at: address.created_at,
                        updated_at: address.updated_at,
                    })
                    .collect()
            })
            .map_err(|_| AppError::DatabaseError)
    }

    fn get_by_id(
        &self,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<AddressInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id);
        let user_id = args.middle_id.ok_or(AppError::ValidationError)?;
        let model_id = args.target_id.ok_or(AppError::ValidationError)?;
        match self.address_repo.find_by_id(model_id, connection) {
            Ok(address) => {
                if address.user_id == user_id {
                    Ok(address).map(|address| AddressInternal {
                        id: address.id,
                        receiver_name: address.receiver_name,
                        phone: address.phone,
                        province: address.province,
                        district: address.district,
                        ward: address.ward,
                        street: address.street,
                        is_default: address.is_default,
                        created_at: address.created_at,
                        updated_at: address.updated_at,
                    })
                } else {
                    return Err(AppError::DatabaseError);
                }
            }

            Err(diesel::result::Error::NotFound) => Err(AppError::UserNotFound),

            Err(_) => Err(AppError::DatabaseError),
        }
    }

    fn delete(&self, args: Args, connection: &mut MysqlConnection) -> Result<(), AppError> {
        let actor_id = self.actor_work(args.actor_id);
        let user_id = args.middle_id.ok_or(AppError::ValidationError)?;
        let result = self
            .address_repo
            .find_all(user_id, connection)
            .map_err(|err| match err {
                diesel::result::Error::NotFound => AppError::NotFound,
                _ => AppError::DatabaseError,
            })?;

        if result.addresses.is_empty() {
            return Err(AppError::NotFound);
        } else if result.addresses.len() == 1 {
            return Err(AppError::ValidationError);
        } else {
            if let Some(address_id) = args.target_id {
                let result_address = self
                    .address_repo
                    .find_by_id(address_id, connection)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => AppError::NotFound,
                        _ => AppError::DatabaseError,
                    })?;
                if result_address.user_id != user_id {
                    return Err(AppError::ValidationError);
                }
                if result_address.is_default == true {
                    return Err(AppError::ValidationError);
                } else {
                    let affected_rows = self
                        .address_repo
                        .delete(address_id, connection)
                        .map_err(|_| AppError::DatabaseError)?;

                    if affected_rows == 0 {
                        return Err(AppError::NotFound);
                    }

                    Ok(())
                }
            } else {
                return Err(AppError::ValidationError);
            }
        }
    }

    fn update(
        &self,
        model_input: AddressUpdateArgs,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<AddressInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id);
        let user_id = args.middle_id.ok_or(AppError::ValidationError)?;
        let model_id = args.target_id.ok_or(AppError::ValidationError)?;
        if model_input.receiver_name.is_none()
            && model_input.phone.is_none()
            && model_input.province.is_none()
            && model_input.district.is_none()
            && model_input.ward.is_none()
            && model_input.street.is_none()
            && model_input.is_default.is_none()
        {
            return Err(AppError::ValidationError);
        }

        if let Some(receiver_name) = model_input.receiver_name.as_ref() {
            if receiver_name.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(phone) = model_input.phone.as_ref() {
            if phone.trim().is_empty()
                || phone.chars().any(char::is_whitespace)
                || phone.len() != 10
                || !phone.starts_with('0')
                || !phone.chars().all(|c| c.is_ascii_digit())
            {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(province) = model_input.province.as_ref() {
            if province.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(district) = model_input.district.as_ref() {
            if district.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(ward) = model_input.ward.as_ref() {
            if ward.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(street) = model_input.street.as_ref() {
            if street.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        let address = self
            .address_repo
            .find_by_id(model_id, connection)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => AppError::UserNotFound,
                _ => AppError::DatabaseError,
            })?;

        if address.user_id == user_id {
            if (model_input.is_default) == Some(true) {
                self.address_repo
                    .set_false(address.user_id, connection)
                    .map_err(|_| AppError::DatabaseError)?;
            }

            let model = UpdateAdress {
                receiver_name: model_input.receiver_name,
                phone: model_input.phone,
                province: model_input.province,
                district: model_input.district,
                ward: model_input.ward,
                street: model_input.street,
                is_default: model_input.is_default,
            };

            self.address_repo
                .update(model_id, model, connection)
                .map_err(|_| AppError::DatabaseError)?;

            self.address_repo
                .find_by_id(model_id, connection)
                .map(|address| AddressInternal {
                    id: address.id,
                    receiver_name: address.receiver_name,
                    phone: address.phone,
                    province: address.province,
                    district: address.district,
                    ward: address.ward,
                    street: address.street,
                    is_default: address.is_default,
                    created_at: address.created_at,
                    updated_at: address.updated_at,
                })
                .map_err(|_| AppError::DatabaseError)
        } else {
            return Err(AppError::ValidationError);
        }
    }

    fn create(
        &self,
        model_input: AddressCreateArgs,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<AddressInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id);
        if model_input.receiver_name.trim().is_empty()
            || model_input.phone.trim().is_empty()
            || model_input.province.trim().is_empty()
            || model_input.ward.trim().is_empty()
            || model_input.street.trim().is_empty()
        {
            return Err(AppError::ValidationError);
        }

        if model_input.phone.chars().any(char::is_whitespace) {
            return Err(AppError::ValidationError);
        }

        let phone = model_input.phone.trim();

        if !phone.starts_with("0")
            || phone.len() != 10
            || !phone.chars().all(|c| c.is_ascii_digit())
        {
            return Err(AppError::ValidationError);
        }

        self.address_repo
            .set_false(model_input.user_id, connection)
            .map_err(|_| AppError::DatabaseError)?;

        let model = NewAddress {
            user_id: model_input.user_id,
            receiver_name: model_input.receiver_name,
            phone: model_input.phone,
            province: model_input.province,
            district: model_input.district,
            ward: model_input.ward,
            street: model_input.street,
            is_default: true,
        };

        self.address_repo
            .create(model, connection)
            .map(|address| AddressInternal {
                id: address.id,
                receiver_name: address.receiver_name,
                phone: address.phone,
                province: address.province,
                district: address.district,
                ward: address.ward,
                street: address.street,
                is_default: address.is_default,
                created_at: address.created_at,
                updated_at: address.updated_at,
            })
            .map_err(|error| {
                println!("CREATE USER DATABASE ERROR: {:?}", error);
                AppError::DatabaseError
            })
    }
}

impl AddressService {
    pub fn new(address_repo: AddressRepository) -> Self {
        Self { address_repo }
    }
}
