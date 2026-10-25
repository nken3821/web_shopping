use diesel::MysqlConnection;

use crate::{
    enums::AppError,
    interface::{BaseRepository, BaseService},
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
    pub is_default: bool,
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

pub struct AddressDeleteArgs {
    pub model_id: Option<i32>,
    pub list_id: Option<Vec<i32>>,
}

impl BaseService for AddressService {
    type InsertModelInput = AddressCreateArgs;
    type OutputModel = AddressInternal;
    type OutputError = AppError;
    type IdType = i32;
    type UpdateModelInput = AddressUpdateArgs;
    type DeleteModelInput = AddressDeleteArgs;

    fn get_all(&self, model_id: Option<i32>, connection: &mut MysqlConnection) -> Result<Vec<AddressInternal>, AppError> {
        let user_id = model_id.ok_or(AppError::InvalidRole)?;
        self.address_repo
            .find_all(user_id, connection)
            .map(|result| {
                result.addresses
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
        connection: &mut MysqlConnection,
        model_id: i32,
    ) -> Result<AddressInternal, AppError> {
        match self.address_repo.find_by_id(model_id, connection) {
            Ok(address) => Ok(address).map(|address| AddressInternal {
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
            }),

            Err(diesel::result::Error::NotFound) => Err(AppError::UserNotFound),

            Err(_) => Err(AppError::DatabaseError),
        }
    }

    fn delete(
        &self,
        connection: &mut MysqlConnection,
        model_id: i32,
        args: AddressDeleteArgs,
    ) -> Result<(), AppError> {
        let result = self
            .address_repo
            .find_all(model_id, connection)
            .map_err(|err| match err {
                diesel::result::Error::NotFound => AppError::NotFound,
                _ => AppError::DatabaseError,
            })?;

        if result.addresses.is_empty() {
            return Err(AppError::NotFound);
        } else if result.addresses.len() == 1 {
            return Err(AppError::ValidationError);
        } else {
            if let Some(address_id) = args.model_id {
                let result_address = self.address_repo.find_by_id(address_id, connection).map_err(|err| match err {
                    diesel::result::Error::NotFound => AppError::NotFound,
                    _ => AppError::DatabaseError,
                })?;
                if result_address.is_default == true {
                    return Err(AppError::ValidationError)
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
                return Err(AppError::DatabaseError);
            }
        }
    }

    fn update(
        &self,
        connection: &mut MysqlConnection,
        model_id: i32,
        args: AddressUpdateArgs,
    ) -> Result<AddressInternal, AppError> {
        if args.receiver_name.is_none()
            && args.phone.is_none()
            && args.province.is_none()
            && args.district.is_none()
            && args.ward.is_none()
            && args.street.is_none()
            && args.is_default.is_none()
        {
            return Err(AppError::ValidationError);
        }

        if let Some(receiver_name) = args.receiver_name.as_ref() {
            if receiver_name.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(phone) = args.phone.as_ref() {
            if phone.trim().is_empty()
                || phone.chars().any(char::is_whitespace)
                || phone.len() != 10
                || !phone.starts_with('0')
                || !phone.chars().all(|c| c.is_ascii_digit())
            {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(province) = args.province.as_ref() {
            if province.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(district) = args.district.as_ref() {
            if district.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(ward) = args.ward.as_ref() {
            if ward.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(street) = args.street.as_ref() {
            if street.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        self.address_repo
            .find_by_id(model_id, connection)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => AppError::UserNotFound,
                _ => AppError::DatabaseError,
            })?;

        let model = UpdateAdress {
            receiver_name: args.receiver_name,
            phone: args.phone,
            province: args.province,
            district: args.district,
            ward: args.ward,
            street: args.street,
            is_default: args.is_default,
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
    }

    fn create(
        &self,
        connection: &mut MysqlConnection,
        args: AddressCreateArgs,
    ) -> Result<AddressInternal, AppError> {
        if args.receiver_name.trim().is_empty()
            || args.phone.trim().is_empty()
            || args.province.trim().is_empty()
            || args.ward.trim().is_empty()
            || args.street.trim().is_empty()
        {
            return Err(AppError::ValidationError);
        }

        if args.phone.chars().any(char::is_whitespace) {
            return Err(AppError::ValidationError);
        }

        let phone = args.phone.trim();

        if !phone.starts_with("0")
            || phone.len() != 10
            || !phone.chars().all(|c| c.is_ascii_digit())
        {
            return Err(AppError::ValidationError);
        }

        self.address_repo
            .set_false(args.user_id, connection)
            .map_err(|_| AppError::DatabaseError)?;

        let model = NewAddress {
            user_id: args.user_id,
            receiver_name: args.receiver_name,
            phone: args.phone,
            province: args.province,
            district: args.district,
            ward: args.ward,
            street: args.street,
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


// Thiết kế lại service de co the dung chung o hai loai user:  Admin va User
// thong nhat thu tu tham so truyen vao 
// sua lai repo neu trong qua trình chuyen dôi co phat sinh loi
// fix lại validator cua emaul o user_service 
// thiet ke lai api, thong nhat chung 1 kieu 
//