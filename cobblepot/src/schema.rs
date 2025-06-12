// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        owner -> Text,
        account_type -> Integer,
        opened_on -> Timestamp,
        closed_on -> Nullable<Timestamp>,
    }
}

diesel::table! {
    balance (id) {
        id -> Integer,
        memo -> Text,
        amount -> Float,
        entered_on -> Timestamp,
        account_id -> Integer,
    }
}

diesel::table! {
    recurring_transactions (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        account_type -> Integer,
        amount -> Float,
        rrule -> Text,
        status -> Integer,
        account_id -> Integer,
    }
}

diesel::joinable!(balance -> account (account_id));
diesel::joinable!(recurring_transactions -> account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    balance,
    recurring_transactions,
);
