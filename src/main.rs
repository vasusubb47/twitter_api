use rocket::{self, get, routes};

mod controlers;
mod database;
mod middlewares;
mod models;
mod schema;
mod utils;

use crate::controlers::{client::*, tweet::*};
use crate::database::PgDbPool;
use crate::middlewares::log_middleware::Logging;
use crate::middlewares::login_middleware::LoginCheck;

#[get("/")]
fn index() -> String {
    "Hello world".to_string()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8888)))
        .manage(PgDbPool::new())
        .mount("/auth", routes![login_client])
        .attach(Logging {})
        .attach(LoginCheck {})
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![get_all_clients, register_client, get_all_tweets,],
        )
        .launch()
        .await?;
    Ok(())
}
