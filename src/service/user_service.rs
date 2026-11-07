use bcrypt::{DEFAULT_COST, hash};
use diesel::MysqlConnection;
use validator::ValidateEmail;

use crate::enums::status::Status;
use crate::enums::{Action, AppError, Model};
use crate::interface::Args;
use crate::interface::{BaseRepository, BaseService};
use crate::internals::UserInternal;
use crate::models::{NewUser, UpdateUser};
use crate::repository::{LOG_REPO, UserRepository};
use crate::service::activity_log_service::{ALogCreateArgs, ActivityLogService};

pub struct UserService {
    user_repo: UserRepository,
    log_ser: ActivityLogService,
}

pub struct UserCreateArgs {
    pub email: String,
    pub fullname: String,
    pub password: String,
    pub phone: String,
}

pub struct UserUpdateArgs {
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl BaseService for UserService {
    type InsertModelInput = UserCreateArgs;
    type UpdateModelInput = UserUpdateArgs;
    type AgrsInput = Args;
    type OutputModel = UserInternal;
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
        let affected_rows = self
            .user_repo
            .delete_more(list_model_id, connection)
            .map_err(|_| AppError::DatabaseError)?;

        if affected_rows == 0 {
            return Err(AppError::UserNotFound);
        }

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::DELETE.as_str().to_string(),
                target_type: Some(Model::USER.as_str().to_string()),
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
    ) -> Result<Vec<UserInternal>, AppError> {
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
        let users = self
            .user_repo
            .find_all(connection)
            .map_err(|_| AppError::DatabaseError)?;
        let result = users
            .into_iter()
            .map(|user| UserInternal {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                user_name: user.user_name,
                phone: user.phone,
                role: user.role,
                created_at: user.created_at,
                updated_at: user.updated_at,
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

        Ok(result)
    }

    fn get_by_id(
        &self,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<UserInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
        let model_id = args.target_id.ok_or(AppError::ValidationError)?;
        let user =
            self.user_repo
                .find_by_id(model_id, connection)
                .map_err(|error| match error {
                    diesel::result::Error::NotFound => AppError::UserNotFound,
                    _ => AppError::DatabaseError,
                })?;
        let result = UserInternal {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            user_name: user.user_name,
            phone: user.phone,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::GETONE.as_str().to_string(),
                target_type: Some(Model::USER.as_str().to_string()),
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
        let model_id = args.target_id.ok_or(AppError::ValidationError)?;
        let affected_rows = self
            .user_repo
            .delete(model_id, connection)
            .map_err(|_| AppError::DatabaseError)?;

        if affected_rows == 0 {
            return Err(AppError::UserNotFound);
        }

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::DELETE.as_str().to_string(),
                target_type: Some(Model::USER.as_str().to_string()),
                target_id: Some(model_id.to_string()),
                description: Some("Delete address by id on user".to_string()),
                status: Status::SUCCESS.as_str().to_string(),
                ip_address: None,
            },
            connection,
        );

        Ok(())
    }

    fn update(
        &self,
        model_input: UserUpdateArgs,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<UserInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
        if model_input.email.is_none()
            && model_input.fullname.is_none()
            && model_input.phone.is_none()
        {
            return Err(AppError::ValidationError);
        }

        // can fix lai Th triet de vi .co van thong qua dc

        if let Some(email) = model_input.email.as_ref() {
            if email.trim().is_empty() || !email.validate_email() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(phone) = model_input.phone.as_ref() {
            if phone.trim().is_empty() || phone.chars().any(char::is_whitespace) {
                return Err(AppError::ValidationError);
            }

            if !phone.starts_with("0")
                || phone.len() != 10
                || !phone.chars().all(|c| c.is_ascii_digit())
            {
                return Err(AppError::ValidationError);
            }

            if !phone.starts_with("0")
                || phone.len() != 10
                || !phone.chars().all(|c| c.is_ascii_digit())
            {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(fullname) = model_input.fullname.as_ref() {
            if fullname.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        let model_id = args.target_id.ok_or(AppError::ValidationError)?;

        self.user_repo
            .find_by_id(model_id, connection)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => AppError::UserNotFound,
                _ => AppError::DatabaseError,
            })?;

        let model = UpdateUser {
            full_name: model_input.fullname,
            phone: model_input.phone,
            email: model_input.email,
        };

        self.user_repo
            .update(model_id, model, connection)
            .map_err(|_| AppError::DatabaseError)?;

        let user = self
            .user_repo
            .find_by_id(model_id, connection)
            .map(|user| UserInternal {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                user_name: user.user_name,
                phone: user.phone,
                role: user.role,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
            .map_err(|_| AppError::DatabaseError)?;

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::UPDATE.as_str().to_string(),
                target_type: Some(Model::USER.as_str().to_string()),
                target_id: Some(model_id.to_string()),
                description: Some("Update address by id on user".to_string()),
                status: Status::SUCCESS.as_str().to_string(),
                ip_address: None,
            },
            connection,
        );

        Ok(user)
    }

    fn create(
        &self,
        model_input: UserCreateArgs,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<UserInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id)?;
        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;
        if model_input.email.trim().is_empty()
            || model_input.phone.trim().is_empty()
            || model_input.password.trim().is_empty()
            || model_input.fullname.trim().is_empty()
        {
            return Err(AppError::ValidationError);
        }

        // can fix lai Th triet de vi .co van thong qua dc

        if !model_input.email.validate_email() {
            return Err(AppError::ValidationError);
        }

        if model_input.email.chars().any(char::is_whitespace)
            || model_input.phone.chars().any(char::is_whitespace)
        {
            return Err(AppError::ValidationError);
        }

        if !model_input.phone.starts_with("0")
            || model_input.phone.len() != 10
            || !model_input.phone.chars().all(|c| c.is_ascii_digit())
        {
            return Err(AppError::ValidationError);
        }

        if model_input.password.len() < 6 {
            return Err(AppError::ValidationError);
        }

        match self.user_repo.find_by_email(&model_input.email, connection) {
            Ok(_) => return Err(AppError::UserAlreadyExists),
            Err(diesel::result::Error::NotFound) => {}
            Err(_) => return Err(AppError::DatabaseError),
        }

        let username = self.get_email_prefix(&model_input.email);

        let password_hash =
            hash(model_input.password, DEFAULT_COST).map_err(|_| AppError::PasswordHashError)?;

        let model = NewUser {
            user_name: username,
            email: model_input.email,
            password_hash,
            phone: model_input.phone,
            full_name: model_input.fullname,
        };

        let user = self
            .user_repo
            .create(model, connection)
            .map(|user| UserInternal {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                user_name: user.user_name,
                phone: user.phone,
                role: user.role,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
            .map_err(|_| AppError::DatabaseError)?;

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::CREATE.as_str().to_string(),
                target_type: Some(Model::USER.as_str().to_string()),
                target_id: None,
                description: Some("Create address by id on user".to_string()),
                status: Status::SUCCESS.as_str().to_string(),
                ip_address: None,
            },
            connection,
        );

        Ok(user)
    }
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self {
            user_repo,
            log_ser: ActivityLogService::new(LOG_REPO),
        }
    }

    fn get_email_prefix(&self, email: &str) -> String {
        email
            .split_once('@')
            .map(|(prefix, _)| prefix.to_string())
            .unwrap_or_default()
    }
}
