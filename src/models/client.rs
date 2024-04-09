use chrono::{NaiveDate, NaiveDateTime};
use diesel::query_dsl::methods::*;
use diesel::{insert_into, ExpressionMethods, Insertable, Queryable, Selectable};
use diesel::{RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::DBConnection;
use crate::schema::client::dsl::*;
use crate::utils::crypto_util::{compare_passcode, hash_passcode_with_salt};

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

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::client)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClientLoginInfo {
    pub id: Uuid,
    pub email: String,
    pub user_name: String,
    pub hasspass: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct BasicClientInfo {
    pub id: Uuid,
    pub email: String,
    pub user_name: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct LoginClientInfo {
    pub email: String,
    pub hasspass: String,
}

#[derive(Debug)]
pub enum ClientErrors {
    ClientEmailAllreadyExists(String),
    ClientUserNameAllreadyExists(String),
    ClientWrongPassword(String),
    ClientNotFound(String),
}

pub fn register_client_fn(
    pool_coon: &mut DBConnection,
    client_info: &RegisterClientInfo,
) -> Result<ClientInfo, ClientErrors> {
    if is_email_being_used(pool_coon, client_info.email.to_owned()) {
        return Err(ClientErrors::ClientEmailAllreadyExists(
            "Email is already used".to_owned(),
        ));
    }

    if is_user_name_being_used(pool_coon, client_info.user_name.to_owned()) {
        return Err(ClientErrors::ClientUserNameAllreadyExists(
            "User Name is already used".to_owned(),
        ));
    }

    let hashed_password = hash_passcode_with_salt(client_info.hasspass.to_owned(), 64);

    let client_info = RegisterClientInfo {
        email: client_info.email.to_owned(),
        first_name: client_info.first_name.to_owned(),
        last_name: client_info.last_name.to_owned(),
        user_name: client_info.user_name.to_owned(),
        gender: client_info.gender.to_owned(),
        dob: client_info.dob,
        hasspass: hashed_password,
    };

    let _ = insert_into(client).values(&client_info).execute(pool_coon);

    let registered_client = get_client_by_email(pool_coon, client_info.email).unwrap();

    Ok(registered_client)
}

pub fn login_client_fn(
    pool_coon: &mut DBConnection,
    login_info: &LoginClientInfo,
) -> Result<BasicClientInfo, ClientErrors> {
    let client_login_info: Result<ClientLoginInfo, diesel::result::Error> = client
        .filter(email.eq(login_info.email.to_owned()))
        .select(ClientLoginInfo::as_select())
        .first(pool_coon);

    match client_login_info {
        Ok(_) => {}
        Err(err) => {
            println!("error while getting client info: {:?}", err);
            return Err(ClientErrors::ClientNotFound(format!(
                "error while getting client info: {:?}",
                err
            )));
        }
    };
    let client_info = client_login_info.unwrap();

    if compare_passcode(
        client_info.hasspass.to_owned(),
        login_info.hasspass.to_owned(),
    ) {
        Ok(BasicClientInfo {
            id: client_info.id,
            email: client_info.email.to_owned(),
            user_name: client_info.user_name.to_owned(),
        })
    } else {
        Err(ClientErrors::ClientWrongPassword(
            "wrong password".to_owned(),
        ))
    }
}

pub fn get_client_by_email(
    pool_coon: &mut DBConnection,
    client_email: String,
) -> Result<ClientInfo, ClientErrors> {
    let client_info: Result<ClientInfo, diesel::result::Error> = client
        .filter(email.eq(client_email))
        .select(ClientInfo::as_select())
        .first(pool_coon);

    match client_info {
        Ok(client_info) => Ok(client_info),
        Err(err) => {
            println!("error while getting client info: {:?}", err);
            Err(ClientErrors::ClientNotFound(format!(
                "error while getting client info: {:?}",
                err
            )))
        }
    }
}

pub fn get_client_by_user_name(
    pool_coon: &mut DBConnection,
    client_user_name: String,
) -> Result<ClientInfo, ClientErrors> {
    let client_info: Result<ClientInfo, diesel::result::Error> = client
        .filter(user_name.eq(client_user_name))
        .select(ClientInfo::as_select())
        .first(pool_coon);

    match client_info {
        Ok(client_info) => Ok(client_info),
        Err(err) => {
            println!("error while getting client info: {:?}", err);
            Err(ClientErrors::ClientNotFound(format!(
                "error while getting client info: {:?}",
                err
            )))
        }
    }
}

pub fn is_email_being_used(pool_coon: &mut DBConnection, client_email: String) -> bool {
    let client_info = get_client_by_email(pool_coon, client_email);
    match client_info {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn is_user_name_being_used(pool_coon: &mut DBConnection, client_user_name: String) -> bool {
    let client_info = get_client_by_user_name(pool_coon, client_user_name);
    match client_info {
        Ok(_) => true,
        Err(_) => false,
    }
}
