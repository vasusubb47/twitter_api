use diesel::query_dsl::methods::SelectDsl;
use diesel::{RunQueryDsl, SelectableHelper};
use rocket::serde::json::Json;
use rocket::{self, get, State};

use crate::database::{DBConnection, PgDbPool};
use crate::models::tweet::Tweet;
use crate::schema::tweet::dsl::*;

#[get("/tweet")]
pub fn get_all_tweets(pool: &State<PgDbPool>) -> Json<Vec<Tweet>> {
    let mut pool_coon: DBConnection = pool.get_connection();

    let tweets = tweet
        .select(Tweet::as_select())
        .load(&mut pool_coon)
        .unwrap();

    Json(tweets)
}
