// use bcrypt::{DEFAULT_COST, hash};
use diesel::MysqlConnection;

use crate::enums::AppError;
use crate::internals::ActivityLogInternal;
use crate::models::NewActivityLog;
use crate::repository::ActivityLogRepository;

pub struct ALogCreateArgs {
    pub actor_id: Option<i32>,
    pub actor_role: Option<String>,
    pub action: String,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub ip_address: Option<String>,
}

pub struct ActivityLogService {
    activity_repo: ActivityLogRepository,
}

impl ActivityLogService {
    pub fn new(activity_repo: ActivityLogRepository) -> Self {
        Self { activity_repo }
    }

    pub fn get_all(
        &self,
        connection: &mut MysqlConnection,
    ) -> Result<Vec<ActivityLogInternal>, AppError> {
        let result = self
            .activity_repo
            .find_all(connection)
            .map_err(|_| AppError::DatabaseError)?;

        let activity_log = result
            .into_iter()
            .map(|activity_log| ActivityLogInternal {
                id: activity_log.id,
                actor_id: activity_log.actor_id,
                actor_role: activity_log.actor_role,
                action: activity_log.action,
                target_type: activity_log.target_type,
                target_id: activity_log.target_id,
                description: activity_log.description,
                status: activity_log.status,
                ip_address: activity_log.ip_address,
                created_at: activity_log.created_at,
            })
            .collect();

        Ok(activity_log)
    }

    pub fn create(
        &self,
        model_input: ALogCreateArgs,
        connection: &mut MysqlConnection,
    ) -> Result<ActivityLogInternal, AppError> {
        let log = NewActivityLog {
            actor_id: model_input.actor_id,
            actor_role: model_input.actor_role,
            action: model_input.action,
            target_type: model_input.target_type,
            target_id: model_input.target_id,
            description: model_input.description,
            status: model_input.status,
            ip_address: model_input.ip_address,
        };

        self.activity_repo
            .create(log, connection)
            .map(|log| ActivityLogInternal {
                id: log.id,
                actor_id: log.actor_id,
                actor_role: log.actor_role,
                action: log.action,
                target_type: log.target_type,
                target_id: log.target_id,
                description: log.description,
                status: log.status,
                ip_address: log.ip_address,
                created_at: log.created_at,
            })
            .map_err(|error| {
                println!("CREATE ACTIVITY LOG DATABASE ERROR: {:?}", error);
                AppError::DatabaseError
            })
    }
}
