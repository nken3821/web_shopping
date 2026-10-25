use bcrypt::{DEFAULT_COST, hash};
use diesel::MysqlConnection;
use validator::ValidateEmail;

use crate::enums::AppError;
use crate::interface::Args;
use crate::interface::{BaseRepository, BaseService};
use crate::internals::UserInternal;
use crate::models::{NewUser, UpdateUser};
use crate::repository::UserRepository;

pub struct UserService {
    user_repo: UserRepository,
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

pub struct UserDeleteArgs {
    pub model_id: Option<i32>,
    pub list_id: Option<Vec<i32>>,
}

impl BaseService for UserService {
    type InsertModelInput = UserCreateArgs;
    type UpdateModelInput = UserUpdateArgs;
    type AgrsInput = Args;
    type OutputModel = UserInternal;
    type OutputError = AppError;

    fn get_all(
        &self,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<Vec<UserInternal>, AppError> {
        let actor_id = self.actor_work(args.actor_id);
        self.user_repo
            .find_all(connection)
            .map(|users| {
                users
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
                    .collect()
            })
            .map_err(|_| AppError::DatabaseError)
    }

    fn get_by_id(
        &self,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<UserInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id);
        let model_id = args.target_id.ok_or(AppError::ValidationError)?;
        match self.user_repo.find_by_id(model_id, connection) {
            Ok(user) => Ok(user).map(|user| UserInternal {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                user_name: user.user_name,
                phone: user.phone,
                role: user.role,
                created_at: user.created_at,
                updated_at: user.updated_at,
            }),

            Err(diesel::result::Error::NotFound) => Err(AppError::UserNotFound),

            Err(_) => Err(AppError::DatabaseError),
        }
    }

    fn delete(&self, args: Args, connection: &mut MysqlConnection) -> Result<(), AppError> {
        let actor_id = self.actor_work(args.actor_id);
        let model_id = args.target_id.ok_or(AppError::ValidationError)?;
        let affected_rows = self
            .user_repo
            .delete(model_id, connection)
            .map_err(|_| AppError::DatabaseError)?;

        if affected_rows == 0 {
            return Err(AppError::UserNotFound);
        }

        Ok(())
    }

    fn update(
        &self,
        model_input: UserUpdateArgs,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<UserInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id);
        if model_input.email.is_none()
            && model_input.fullname.is_none()
            && model_input.phone.is_none()
        {
            return Err(AppError::ValidationError);
        }

        // can fix lai Th triet de vi .co van thong qua dc
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

        self.user_repo
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
            .map_err(|_| AppError::DatabaseError)
    }

    fn create(
        &self,
        model_input: UserCreateArgs,
        args: Args,
        connection: &mut MysqlConnection,
    ) -> Result<UserInternal, AppError> {
        let actor_id = self.actor_work(args.actor_id);
        if model_input.email.trim().is_empty()
            || model_input.phone.trim().is_empty()
            || model_input.password.trim().is_empty()
            || model_input.fullname.trim().is_empty()
        {
            return Err(AppError::ValidationError);
        }

        // can fix lai Th triet de vi .co van thong qua dc
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

        self.user_repo
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
            .map_err(|error| {
                println!("CREATE USER DATABASE ERROR: {:?}", error);
                AppError::DatabaseError
            })
    }
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    fn get_email_prefix(&self, email: &str) -> String {
        email
            .split_once('@')
            .map(|(prefix, _)| prefix.to_string())
            .unwrap_or_default()
    }
}
