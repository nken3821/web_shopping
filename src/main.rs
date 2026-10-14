use rocket::{get, routes, serde::json::Json};
use serde_json::{Value, json};


mod config;


#[get("/")]
fn hello_world() -> Json<Value> {
    let hello = Json(json!(
        {
            "init" : "hello world"
        }
    ));
   hello
}

#[rocket::main]
async fn main() {
    rocket::build()
        .manage(config::connect())
        .mount("/", routes![hello_world])
        .launch().await.unwrap();
}


