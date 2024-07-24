// @generated automatically by Diesel CLI.

diesel::table! {
    loan_applicants (id) {
        id -> Int4,
        user_id -> Int4,
        loan_id -> Int4,
        application_status -> Varchar,
        application_date -> Nullable<Timestamp>,
        approved_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    loan_repayments (id) {
        id -> Int4,
        application_id -> Int4,
        repayment_amount -> Int4,
        repayment_date -> Nullable<Timestamp>,
        status -> Varchar,
    }
}

diesel::table! {
    loans (id) {
        id -> Int4,
        loan_type -> Varchar,
        amount -> Int4,
        interest_rate -> Varchar,
        term_length -> Int4,
        description -> Nullable<Text>,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(loan_applicants -> loans (loan_id));
diesel::joinable!(loan_applicants -> users (user_id));
diesel::joinable!(loan_repayments -> loan_applicants (application_id));
diesel::joinable!(loans -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    loan_applicants,
    loan_repayments,
    loans,
    users,
);
