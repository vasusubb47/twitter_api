use diesel::query_dsl::methods::SelectDsl;
use diesel::{insert_into, RunQueryDsl, SelectableHelper};
use rocket::{self, get, post, serde::json::Json, State};
use uuid::Uuid;

use crate::database::{DBConnection, PgDbPool};
use crate::schema::client::dsl::*;
use crate::{ClientInfo, RegisterClientInfo};

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
pub fn register_client(pool: &State<PgDbPool>, reg_user: Json<RegisterClientInfo>) -> Json<Uuid> {
    let mut pool_coon: DBConnection = pool.get_connection();

    println!("{:#?}", reg_user);

    let new_client: Uuid = insert_into(client)
        .values(&reg_user.0)
        .returning(id)
        .get_result(&mut pool_coon)
        .unwrap();

    Json(new_client)
}
