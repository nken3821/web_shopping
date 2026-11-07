use bcrypt::verify;
use diesel::MysqlConnection;

use crate::{
    enums::{Action, AppError, Model, status::Status},
    internals::UserInternal,
    repository::{LOG_REPO, UserRepository},
    service::{ActivityLogService, activity_log_service::ALogCreateArgs},
};

pub struct LoginService {
    user_repo: UserRepository,
    log_ser: ActivityLogService,
}

impl LoginService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self {
            user_repo,
            log_ser: ActivityLogService::new(LOG_REPO),
        }
    }

    pub fn login(
        &self,
        connection: &mut MysqlConnection,
        password: &str,
        email: &str,
    ) -> Result<UserInternal, AppError> {
        let user =
            self.user_repo
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

        let actor_id = result.id;

        let role = self
            .user_repo
            .get_role(actor_id, connection)
            .map_err(|_| AppError::DatabaseError)?;

        let _log = self.log_ser.create(
            ALogCreateArgs {
                actor_id: Some(actor_id),
                actor_role: Some(role),
                action: Action::LOGIN.as_str().to_string(),
                target_type: Some(Model::USER.as_str().to_string()),
                target_id: None,
                description: Some("Get list address on user".to_string()),
                status: Status::SUCCESS.as_str().to_string(),
                ip_address: None,
            },
            connection,
        );

        Ok(result)
    }
}
