use diesel::MysqlConnection;

pub trait BaseService {
    type InsertModelInput;
    type OutputModel;
    type OutputError;
    type IdType;
    type UpdateModelInput;
    type DeleteModelInput;

    fn get_all(
        &self,
        model_id: Option< Self::IdType>,
        connection: &mut MysqlConnection,
    ) -> Result<Vec<Self::OutputModel>, Self::OutputError>;

    fn get_by_id(
        &self,
        connection: &mut MysqlConnection,
        model_id: Self::IdType,
    ) -> Result<Self::OutputModel, Self::OutputError>;

    fn delete(
        &self,
        connection: &mut MysqlConnection,
        model_id: Self::IdType,
        args: Self::DeleteModelInput,
    ) -> Result<(), Self::OutputError>;

    fn update(
        &self,
        connection: &mut MysqlConnection,
        model_id: Self::IdType,
        args: Self::UpdateModelInput,
    ) -> Result<Self::OutputModel, Self::OutputError>;

    fn create(
        &self,
        connection: &mut MysqlConnection,
        args: Self::InsertModelInput,
    ) -> Result<Self::OutputModel, Self::OutputError>;
}
