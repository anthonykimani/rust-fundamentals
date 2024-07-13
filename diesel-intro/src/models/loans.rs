use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Loans {
    pub amount: i32,
    pub borrower_name: String,
    pub loan_date: String,
    pub due_date: String,
    pub user_id: i32
}
