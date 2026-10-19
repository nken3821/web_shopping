use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::{models::{NewUser, UpdateUser, User}, schema::users::{self}};
use crate::interface::BaseRepository;

pub struct UserRepository;

impl BaseRepository for UserRepository {

    type IdType = i32;
    type InsertModel = NewUser;
    type OutputModel = User;
    type UpdateModel = UpdateUser;

    
    fn find_by_id(&self, model_id: i32, connection: &mut MysqlConnection) -> QueryResult<User> {
        users::table
            .find(model_id)
            .select(User::as_select())
            .first(connection)
    }

    fn create(&self, model: NewUser, connection: &mut MysqlConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&model)
            .execute(connection)?;

        match self.get_last_insert_id(connection) {
            Ok(result_id) => {
                users::table
                    .find(result_id as i32)
                    .select(User::as_select())
                    .first(connection)
            },
            Err(db_error) => {
                println!("Error when LAST_INSERT_ID: {:?}", db_error);
                Err(db_error)
            }
        }
    }

    fn find_all(&self, connection : &mut MysqlConnection) -> QueryResult<Vec<User>> {
        users::table
            .select(User::as_select())
            .load(connection)
    }

    
    fn update(&self, model_id: i32, model: UpdateUser, connection: &mut MysqlConnection) -> QueryResult<usize> {
        diesel::update(users::table.find(model_id))
            .set(&model)
            .execute(connection)
    }

    fn delete(&self, model_id: i32, connection : &mut MysqlConnection) -> QueryResult<usize> {
        diesel::delete(users::table.find(model_id))
            .execute(connection)
    }

}

impl UserRepository {
    pub fn find_by_email(&self, email: &str, connection: &mut MysqlConnection) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .first(connection)
    }
}