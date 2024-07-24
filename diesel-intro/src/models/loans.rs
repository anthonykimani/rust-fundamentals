use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loan {
    pub id: i32,
    pub loan_type: String,
    pub amount: i32,
    pub interest_rate: String,
    pub term_length: i32,
    pub description: Option<String>,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoan {
    pub loan_type: String,
    pub amount: i32,
    pub interest_rate: String,
    pub term_length: i32,
    pub user_id: i32,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateLoanForm {
    pub loan_type: String,
    pub amount: i32,
    pub interest_rate: String,
    pub term_length: i32,
    pub description: Option<String>,
}
