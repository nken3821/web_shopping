use diesel::MysqlConnection;
use bcrypt::{verify};

use crate::{enums::AppError, internals::UserInternal, repository::UserRepository};

pub struct LoginService {
    user_repo: UserRepository,
}

impl LoginService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub fn login(
        &self,
        connection: &mut MysqlConnection,
        password: &str,
        email: &str,
    ) -> Result<UserInternal, AppError> {
        let user = self
            .user_repo
            .find_by_email(email, connection)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => AppError::UserNotFound,
                _ => AppError::DatabaseError,
            })?;

        let valid =
            verify(password, &user.password_hash).map_err(|_| AppError::AuthenticationError)?;

        if !valid {
            return Err(AppError::AuthenticationError);
        }

        Ok(user).map(|user| UserInternal {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                user_name: user.user_name,
                phone: user.phone,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
    }

}