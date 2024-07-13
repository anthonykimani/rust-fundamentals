use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Users {
    pub id: i32,
    pub name: String,
    pub firstname: String,
    pub age: i32,
    pub email: String,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct NewUsers {
    pub name: String,
    pub firstname: String,
    pub age: i32,
    pub email: String,
}