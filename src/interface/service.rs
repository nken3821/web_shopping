use diesel::MysqlConnection;

use crate::enums::AppError;

#[derive(Default)]
pub struct Args {
    pub actor_id: Option<i32>,
    pub middle_id: Option<i32>,
    pub target_id: Option<i32>,
    pub targets_id: Option<Vec<i32>>,
}

pub trait BaseService {
    type InsertModelInput;
    type UpdateModelInput;
    type AgrsInput;
    type OutputModel;
    type OutputError;
<<<<<<< HEAD
    type IdType;
    type UpdateModelInput;

    fn get_all(
        &self,
=======

    fn get_all(
        &self,
        args: Self::AgrsInput,
>>>>>>> f6791a0 (feat: add address management functionality with CRUD operations)
        connection: &mut MysqlConnection,
    ) -> Result<Vec<Self::OutputModel>, Self::OutputError>;

    fn get_by_id(
        &self,
        args: Self::AgrsInput,
        connection: &mut MysqlConnection,
    ) -> Result<Self::OutputModel, Self::OutputError>;

    fn delete(
        &self,
        args: Self::AgrsInput,
        connection: &mut MysqlConnection,
<<<<<<< HEAD
=======
>>>>>>> f6791a0 (feat: add address management functionality with CRUD operations)
    ) -> Result<(), Self::OutputError>;

    fn update(
        &self,
        model_input: Self::UpdateModelInput,
        args: Self::AgrsInput,
        connection: &mut MysqlConnection,
    ) -> Result<Self::OutputModel, Self::OutputError>;

    fn create(
        &self,
        model_input: Self::InsertModelInput,
        args: Self::AgrsInput,
        connection: &mut MysqlConnection,
    ) -> Result<Self::OutputModel, Self::OutputError>;

    fn actor_work(&self, actor_id: Option<i32>) -> Result<i32, AppError> {
        let actor_id = actor_id.ok_or(AppError::AuthenticationError)?;
        Ok(actor_id)
    }
}
