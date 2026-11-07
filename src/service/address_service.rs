use diesel::MysqlConnection;

use crate::{
    enums::{Action, AppError, Model, status::Status},
    interface::{Args, BaseRepository, BaseService},
    internals::AddressInternal,
    models::{NewAddress, UpdateAdress},
    repository::{AddressRepository, LOG_REPO, UserRepository},
    service::activity_log_service::{ALogCreateArgs, ActivityLogService},
};

pub struct AddressService {
    address_repo: AddressRepository,
    log_ser: ActivityLogService,
    user_repo: UserRepository,
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

    fn delete_more_model(
        &self,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<(), AppError> {
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
        let list_model_id = args.targets_id.ok_or(AppError::ValidationError)?;
        let target_id_model: String = format!("{:?}", list_model_id);
        let user_id = args.middle_id.ok_or(AppError::ValidationError)?;
        let result = self
            .address_repo
            .find_all(user_id, connection)
            .map_err(|_| AppError::DatabaseError)?;

        if result.user.id != user_id {
            return Err(AppError::ValidationError);
        }

        if list_model_id.is_empty() {
            return Err(AppError::ValidationError);
        }

        if result.addresses.len() <= list_model_id.len() {
            return Err(AppError::ValidationError);
        } else {
            for address_id in list_model_id.clone() {
                let address = self
                    .address_repo
                    .find_by_id(address_id, connection)
                    .map_err(|error| match error {
                        diesel::result::Error::NotFound => AppError::NotFound,
                        _ => AppError::DatabaseError,
                    })?;
                if address.is_default == true {
                    return Err(AppError::ValidationError);
                } else {
                    continue;
                }
            }
        }

        let affected_rows = self
            .address_repo
            .delete_more(list_model_id, connection)
            .map_err(|_| AppError::DatabaseError)?;

        if affected_rows == 0 {
            return Err(AppError::UserNotFound);
        }

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::DELETELIST.as_str().to_string(),
                target_type: Some(Model::ADDRESS.as_str().to_string()),
                target_id: Some(target_id_model),
                description: Some("Delete more address by id on user".to_string()),
                status: Status::SUCCESS.as_str().to_string(),
                ip_address: None,
            },
            connection,
        );

        Ok(())
    }

    fn get_all(
        &self,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<Vec<AddressInternal>, AppError> {
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
        let user_id = args.middle_id.ok_or(AppError::ValidationError)?;
        let result = self
            .address_repo
            .find_all(user_id, connection)
            .map_err(|_| AppError::DatabaseError)?;

        let addresses = result
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
            .collect();

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::GETLIST.as_str().to_string(),
                target_type: Some(Model::ADDRESS.as_str().to_string()),
                target_id: None,
                description: Some("Get list address on user".to_string()),
                status: Status::SUCCESS.as_str().to_string(),
                ip_address: None,
            },
            connection,
        );

        Ok(addresses)
    }

    fn get_by_id(
        &self,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<AddressInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
        let user_id = args.middle_id.ok_or(AppError::ValidationError)?;
        let model_id = args.target_id.ok_or(AppError::ValidationError)?;

        let address = self
            .address_repo
            .find_by_id(model_id, connection)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => AppError::NotFound,
                _ => AppError::DatabaseError,
            })?;

        if address.user_id != user_id {
            return Err(AppError::DatabaseError);
        }

        let result = AddressInternal {
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
        };

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::GETONE.as_str().to_string(),
                target_type: Some(Model::ADDRESS.as_str().to_string()),
                target_id: Some(model_id.to_string()),
                description: Some("Get address by id on user".to_string()),
                status: Status::SUCCESS.as_str().to_string(),
                ip_address: None,
            },
            connection,
        );

        Ok(result)
    }

    fn delete(&self, args: Args, connection: &mut MysqlConnection) -> Result<(), AppError> {
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
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

                    let _log = self.log_ser.create(
                        ALogCreateArgs {
                            actor_id: Some(actor_id),
                            actor_role: Some(role),
                            action: Action::DELETE.as_str().to_string(),
                            target_type: Some(Model::ADDRESS.as_str().to_string()),
                            target_id: Some(address_id.to_string()),
                            description: Some("Delete address by id on user".to_string()),
                            status: Status::SUCCESS.as_str().to_string(),
                            ip_address: None,
                        },
                        connection,
                    );

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
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
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

            let address = self
                .address_repo
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
                .map_err(|error| match error {
                    diesel::result::Error::NotFound => AppError::NotFound,
                    _ => AppError::DatabaseError,
                })?;

            let _log = self.log_ser.create(
                ALogCreateArgs {
                    actor_id: Some(actor_id),
                    actor_role: Some(role),
                    action: Action::UPDATE.as_str().to_string(),
                    target_type: Some(Model::ADDRESS.as_str().to_string()),
                    target_id: Some(model_id.to_string()),
                    description: Some("Update address by id on user".to_string()),
                    status: Status::SUCCESS.as_str().to_string(),
                    ip_address: None,
                },
                connection,
            );
            Ok(address)
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
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
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

        let address = self
            .address_repo
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
            .map_err(|_| AppError::DatabaseError)?;

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::CREATE.as_str().to_string(),
                target_type: Some(Model::ADDRESS.as_str().to_string()),
                target_id: None,
                description: Some("Create address by id on user".to_string()),
                status: Status::SUCCESS.as_str().to_string(),
                ip_address: None,
            },
            connection,
        );

        Ok(address)
    }
}

impl AddressService {
    pub fn new(address_repo: AddressRepository) -> Self {
        Self {
            address_repo,
            log_ser: ActivityLogService::new(LOG_REPO),
            user_repo: UserRepository,
        }
    }
}
