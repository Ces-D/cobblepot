// @generated automatically by Diesel CLI.

diesel::table! {
    account (account_code) {
        account_code -> Integer,
        account_variant -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_on -> Text,
        closed_on -> Nullable<Text>,
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
        amount -> Text,
        currency -> Text,
        created_on -> Text,
    }
}

diesel::joinable!(journal -> account (account_code));
diesel::joinable!(journal -> transaction_event (transaction_code));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    journal,
    transaction_event,
);
