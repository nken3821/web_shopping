use diesel::{
    MysqlConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper,
};

use crate::{interface::BaseRepository, models::{Address, NewAddress, UpdateAdress, address::UserWithAddresses}, routes::USER_REPO, schema::addresses};
use diesel::prelude::*;
pub struct AddressRepository;

impl BaseRepository for AddressRepository {

    type IdType = i32;
    type InsertModel = NewAddress;
    type OutputModel = Address;
    type UpdateModel = UpdateAdress;
    
    fn find_by_id(&self, model_id: i32, connection: &mut MysqlConnection) -> QueryResult<Address> {
        addresses::table
            .find(model_id)
            .select(Address::as_select())
            .first(connection)
    }

    fn create(&self, model: NewAddress, connection: &mut MysqlConnection) -> QueryResult<Address> {
        diesel::insert_into(addresses::table)
            .values(&model)
            .execute(connection)?;

        match self.get_last_insert_id(connection) {
            Ok(result_id) => addresses::table
                .find(result_id as i32)
                .select(Address::as_select())
                .first(connection),
            Err(db_error) => {
                println!("Error when LAST_INSERT_ID: {:?}", db_error);
                Err(db_error)
            }
        }
    }

    fn update(
        &self,
        model_id: i32,
        model: UpdateAdress,
        connection: &mut MysqlConnection,
    ) -> QueryResult<usize> {
        diesel::update(addresses::table.find(model_id))
            .set(&model)
            .execute(connection)
    }

    fn delete(&self, model_id: i32, connection: &mut MysqlConnection) -> QueryResult<usize> {
        diesel::delete(addresses::table.find(model_id)).execute(connection)
    }
}


impl AddressRepository {
    pub fn find_all(&self, user_id: i32, connection: &mut MysqlConnection) -> QueryResult<UserWithAddresses> {
        let user = USER_REPO.find_by_id(user_id, connection)?;

        let user_adresses = Address::belonging_to(&user).load::<Address>(connection)?;

        Ok(UserWithAddresses {
            user,
            addresses: user_adresses,
        })

    }

    pub fn set_false(&self, user_id: i32, connection: &mut MysqlConnection) -> QueryResult<usize>{
        diesel::update(addresses::table.filter(addresses::user_id.eq(user_id)))
            .set(addresses::is_default.eq(false))
            .execute(connection)
        
    }
}

    