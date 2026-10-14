use std::{env, ops::Deref};

use diesel::{MysqlConnection, r2d2::{self, ConnectionManager}};
use rocket::{Request, State, http::Status, request::{self, FromRequest, Outcome}};

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;


pub fn connect() -> Pool {
    let _ = dotenvy::dotenv();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}

pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

#[rocket::async_trait]
impl <'r> FromRequest<'r> for Connection {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = match request.guard::<&State<Pool>>().await {
            Outcome::Success(pool) => pool,
            _ => return Outcome::Error((Status::InternalServerError, ())),
        };
        match pool.get() {
            Ok(coon) => Outcome::Success(Connection(coon)),
            Err(_) => Outcome::Error((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}