use diesel::{MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};

use diesel::prelude::*;

use crate::interface::repository::LastInsertId;
use crate::models::activity_log::{ActivityLog, NewActivityLog};
use crate::schema::activity_logs;

pub struct ActivityLogRepository;

impl ActivityLogRepository {
    fn find_by_id(
        &self,
        model_id: i32,
        connection: &mut MysqlConnection,
    ) -> QueryResult<ActivityLog> {
        activity_logs::table
            .find(model_id)
            .select(ActivityLog::as_select())
            .first(connection)
    }

    pub fn create(
        &self,
        model: NewActivityLog,
        connection: &mut MysqlConnection,
    ) -> QueryResult<ActivityLog> {
        diesel::insert_into(activity_logs::table)
            .values(&model)
            .execute(connection)?;

        let result_id = self._get_last_insert_id(connection)?;

        activity_logs::table
            .find(result_id as i32)
            .select(ActivityLog::as_select())
            .first(connection)
    }

    fn _get_last_insert_id(
        &self,
        connection: &mut MysqlConnection,
    ) -> Result<u64, diesel::result::Error> {
        let result: LastInsertId =
            diesel::sql_query("SELECT LAST_INSERT_ID() as id").get_result(connection)?;
        Ok(result.id)
    }

    pub fn find_all(&self, connection: &mut MysqlConnection) -> QueryResult<Vec<ActivityLog>> {
        activity_logs::table
            .select(ActivityLog::as_select())
            .load(connection)
    }
}
