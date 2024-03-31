use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::client)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub gender: Option<String>,
    pub dob: NaiveDate,
    pub hasspass: String,
    pub followers_count: i32,
    pub following_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::client)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClientInfo {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub gender: Option<String>,
    pub dob: NaiveDate,
    pub followers_count: i32,
    pub following_count: i32,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::client)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RegisterClientInfo {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub gender: Option<String>,
    pub dob: NaiveDate,
    pub hasspass: String,
}
