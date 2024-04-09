use diesel::query_dsl::methods::SelectDsl;
use diesel::{RunQueryDsl, SelectableHelper};
use rocket::{self, get, post, response::status::BadRequest, serde::json::Json, State};
// use uuid::Uuid;

use crate::database::{DBConnection, PgDbPool};
use crate::models::client::*;
use crate::schema::client::dsl::*;

#[get("/client")]
pub fn get_all_clients(pool: &State<PgDbPool>) -> Json<Vec<ClientInfo>> {
    let mut pool_coon: DBConnection = pool.get_connection();

    let clients = client
        .select(ClientInfo::as_select())
        .load(&mut pool_coon)
        .unwrap();

    Json(clients)
}

#[post("/register", format = "json", data = "<reg_user>")]
pub fn register_client(
    pool: &State<PgDbPool>,
    reg_user: Json<RegisterClientInfo>,
) -> Result<Json<ClientInfo>, BadRequest<String>> {
    let mut pool_coon: DBConnection = pool.get_connection();

    let new_client = register_client_fn(&mut pool_coon, &reg_user.0);

    match new_client {
        Ok(client_info) => Ok(Json(client_info)),
        Err(err) => match err {
            ClientErrors::ClientEmailAllreadyExists(err) => Err(BadRequest(err)),
            ClientErrors::ClientUserNameAllreadyExists(err) => Err(BadRequest(err)),
            ClientErrors::ClientNotFound(err_info) => Err(BadRequest(err_info)),
            _ => panic!("never runs"),
        },
    }
}

#[post("/login", format = "json", data = "<login_info>")]
pub fn login_client(
    pool: &State<PgDbPool>,
    login_info: Json<LoginClientInfo>,
) -> Result<Json<BasicClientInfo>, BadRequest<String>> {
    let mut pool_coon: DBConnection = pool.get_connection();

    let basic_login_info = login_client_fn(&mut pool_coon, &login_info);

    match basic_login_info {
        Ok(basic_login_info) => Ok(Json(basic_login_info)),
        Err(err) => match err {
            ClientErrors::ClientWrongPassword(err_info) => Err(BadRequest(err_info)),
            ClientErrors::ClientNotFound(err_info) => Err(BadRequest(err_info)),
            _ => panic!("never runs"),
        },
    }
}
