use crate::config::Connection;
use crate::dtos::activity::activity_response::ActivityResponse;
use crate::guards::auth_guard::AdminUser;
use crate::routes::ACTIVITY_REPO;
use crate::service::ActivityLogService;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/")]
pub fn get_activities(
    _user: AdminUser,
    connection: Connection,
) -> Result<Json<Vec<ActivityResponse>>, Status> {
    let mut connection = connection.0;

    let activity_ser = ActivityLogService::new(ACTIVITY_REPO);

    activity_ser
        .get_all(&mut connection)
        .map(|activities| {
            Json({
                activities
                    .into_iter()
                    .map(|activity| ActivityResponse {
                        id: activity.id,
                        actor_id: activity.actor_id,
                        actor_role: activity.actor_role,
                        action: activity.action,
                        target_type: activity.target_type,
                        target_id: activity.target_id,
                        description: activity.description,
                        status: activity.status,
                        ip_address: activity.ip_address,
                        created_at: activity.created_at,
                    })
                    .collect()
            })
        })
        .map_err(|error| error.status())
}
