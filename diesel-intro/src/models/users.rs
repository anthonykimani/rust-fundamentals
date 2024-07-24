use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub password: String,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct NewUsers {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub password: String
}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub  email: String,
    pub  password: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub age: String,
    pub password: String
}