use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::available_loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvailableLoan {
    pub id: i32,
    pub loan_type: String,
    pub amount: i32,
    pub interest_rate: String,
    pub term_length: i32,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
