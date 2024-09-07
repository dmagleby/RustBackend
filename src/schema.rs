// @generated automatically by Diesel CLI.

diesel::table! {
    cases (id) {
        id -> Int4,
        lawyer_id -> Int4,
        title -> Varchar,
        description -> Text,
        funding_goal -> Numeric,
        current_funding -> Numeric,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    investments (id) {
        id -> Int4,
        user_id -> Int4,
        case_id -> Int4,
        amount -> Numeric,
        investment_type -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        user_id -> Int4,
        amount -> Numeric,
        transaction_type -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(cases -> users (lawyer_id));
diesel::joinable!(investments -> cases (case_id));
diesel::joinable!(investments -> users (user_id));
diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cases,
    investments,
    transactions,
    users,
);
