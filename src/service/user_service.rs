use bcrypt::{DEFAULT_COST, hash};
use diesel::MysqlConnection;
use validator::ValidateEmail;

use crate::enums::AppError;
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
    type OutputModel = UserInternal;
    type OutputError = AppError;
    type IdType = i32;
    type UpdateModelInput = UserUpdateArgs;
    type DeleteModelInput = UserDeleteArgs;

    fn get_all(&self, _model_id: Option<i32>, connection: &mut MysqlConnection) -> Result<Vec<UserInternal>, AppError> {
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
        connection: &mut MysqlConnection,
        model_id: i32,
    ) -> Result<UserInternal, AppError> {
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

    fn delete(&self, connection: &mut MysqlConnection, model_id: i32, _args: UserDeleteArgs) -> Result<(), AppError> {
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
        connection: &mut MysqlConnection,
        model_id: i32,
        args: UserUpdateArgs,
    ) -> Result<UserInternal, AppError> {
        if args.email.is_none() && args.fullname.is_none() && args.phone.is_none() {
            return Err(AppError::ValidationError);
        }

        // can fix lai Th triet de vi .co van thong qua dc

        if let Some(email) = args.email.as_ref() {
            if email.trim().is_empty() || !email.validate_email() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(phone) = args.phone.as_ref() {
            if phone.trim().is_empty() || phone.chars().any(char::is_whitespace) {
                return Err(AppError::ValidationError);
            }

            if !phone.starts_with("0")
                || phone.len() != 10
                || !phone.chars().all(|c| c.is_ascii_digit())
            {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(fullname) = args.fullname.as_ref() {
            if fullname.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        self.user_repo
            .find_by_id(model_id, connection)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => AppError::UserNotFound,
                _ => AppError::DatabaseError,
            })?;

        let model = UpdateUser {
            full_name: args.fullname,
            phone: args.phone,
            email: args.email,
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
        connection: &mut MysqlConnection,
        args: UserCreateArgs,
    ) -> Result<UserInternal, AppError> {
        if args.email.trim().is_empty()
            || args.phone.trim().is_empty()
            || args.password.trim().is_empty()
            || args.fullname.trim().is_empty()
        {
            return Err(AppError::ValidationError);
        }

        // can fix lai Th triet de vi .co van thong qua dc

        if !args.email.validate_email() {
            return Err(AppError::ValidationError);
        }

        if args.email.chars().any(char::is_whitespace)
            || args.phone.chars().any(char::is_whitespace)
        {
            return Err(AppError::ValidationError);
        }

        if !args.phone.starts_with("0")
            || args.phone.len() != 10
            || !args.phone.chars().all(|c| c.is_ascii_digit())
        {
            return Err(AppError::ValidationError);
        }

        if args.password.len() < 6 {
            return Err(AppError::ValidationError);
        }

        match self.user_repo.find_by_email(&args.email, connection) {
            Ok(_) => return Err(AppError::UserAlreadyExists),
            Err(diesel::result::Error::NotFound) => {}
            Err(_) => return Err(AppError::DatabaseError),
        }

        let username = self.get_email_prefix(&args.email);

        let password_hash =
            hash(args.password, DEFAULT_COST).map_err(|_| AppError::PasswordHashError)?;

        let model = NewUser {
            user_name: username,
            email: args.email,
            password_hash,
            phone: args.phone,
            full_name: args.fullname,
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
