use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::loan_applicants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoanApplicants {
    pub id: i32,
    pub application_status: String,
    pub application_date: chrono::NaiveDateTime,
    pub approved_date: chrono::NaiveDateTime
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::loan_applicants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoanApplicants {
    pub application_status: String,
    pub application_date: chrono::NaiveDateTime,
    pub approved_date: chrono::NaiveDateTime
}