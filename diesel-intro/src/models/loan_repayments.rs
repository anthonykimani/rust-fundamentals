use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::loan_repayments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoanRepayments {
    pub id: i32,
    pub repayment_amount: i32,
    pub repayment_date: chrono::NaiveDateTime,
    pub status: String
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::loan_repayments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoanRepayments {
    pub repayment_amount: i32,
    pub repayment_date: chrono::NaiveDateTime,
    pub status: String
}