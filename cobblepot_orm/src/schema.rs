// @generated automatically by Diesel CLI.

diesel::table! {
    account (account_code) {
        account_code -> Integer,
        account_variant -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_on -> Timestamp,
        closed_on -> Nullable<Timestamp>,
    }
}

diesel::table! {
    balance (id) {
        id -> Integer,
        account_code -> Integer,
        current_balance -> Integer,
        journal_entry -> Integer,
    }
}

diesel::table! {
    journal (entry_code) {
        entry_code -> Integer,
        memo -> Text,
        account_code -> Integer,
        transaction_code -> Integer,
    }
}

diesel::table! {
    transaction_event (transaction_code) {
        transaction_code -> Integer,
        amount -> Integer,
        currency -> Text,
        created_on -> Timestamp,
    }
}

diesel::joinable!(balance -> account (account_code));
diesel::joinable!(balance -> journal (journal_entry));
diesel::joinable!(balance -> transaction_event (current_balance));
diesel::joinable!(journal -> account (account_code));
diesel::joinable!(journal -> transaction_event (transaction_code));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    balance,
    journal,
    transaction_event,
);
