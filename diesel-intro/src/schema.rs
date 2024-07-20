// @generated automatically by Diesel CLI.

diesel::table! {
    loans (id) {
        id -> Int4,
        amount -> Int4,
        borrower_name -> Text,
        loan_date -> Text,
        due_date -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        #[max_length = 150]
        firstname -> Varchar,
        age -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(loans -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    loans,
    posts,
    users,
);
