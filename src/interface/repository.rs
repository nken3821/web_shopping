use diesel::prelude::*;
use diesel::{
    MysqlConnection, QueryResult,
    deserialize::QueryableByName,
    sql_types::{BigInt, Unsigned},
};

#[derive(QueryableByName)]
pub struct LastInsertId {
    #[diesel(sql_type = Unsigned<BigInt>)]
    id: u64,
}

pub trait BaseRepository {
    type IdType;
    type InsertModel;
    type OutputModel;
    type UpdateModel;

    fn get_last_insert_id(
        &self,
        connection: &mut MysqlConnection,
    ) -> Result<u64, diesel::result::Error> {
        let result: LastInsertId =
            diesel::sql_query("SELECT LAST_INSERT_ID() as id").get_result(connection)?;
        Ok(result.id)
    }

    // fn find_all(&self, connection: &mut MysqlConnection) -> QueryResult<Vec<Self::OutputModel>>;

    fn create(
        &self,
        model: Self::InsertModel,
        connection: &mut MysqlConnection,
    ) -> QueryResult<Self::OutputModel>;

    fn find_by_id(
        &self,
        model_id: Self::IdType,
        connection: &mut MysqlConnection,
    ) -> QueryResult<Self::OutputModel>;

    fn update(
        &self,
        model_id: Self::IdType,
        model: Self::UpdateModel,
        connection: &mut MysqlConnection,
    ) -> QueryResult<usize>;

    fn delete(
        &self,
        model_id: Self::IdType,
        connection: &mut MysqlConnection,
    ) -> QueryResult<usize>;
}
