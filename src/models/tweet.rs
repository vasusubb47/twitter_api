use chrono::NaiveDateTime;
use diesel::{FromSqlRow, Queryable, Selectable};
use diesel_json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tweet)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tweet {
    pub id: Uuid,
    pub cid: Uuid,
    pub tweet_body: String,
    pub reply_tid: Option<Uuid>,
    pub re_tid: Option<Uuid>,
    pub likes: i32,
    pub hash_tags: Option<Json<HashTag>>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromSqlRow)]
pub struct HashTag {
    pub tags: Vec<String>,
}
