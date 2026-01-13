// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        owner -> Text,
        account_type -> Integer,
        opened_on -> Integer,
        closed_on -> Nullable<Integer>,
    }
}

diesel::table! {
    balance (id) {
        id -> Integer,
        memo -> Text,
        amount -> Float,
        entered_on -> Integer,
        account_id -> Integer,
    }
}

diesel::table! {
    budget (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        anticipated_amount -> Float,
        budget_recurrence_id -> Nullable<Integer>,
    }
}

diesel::table! {
    budget_item (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        amount -> Float,
        budget_id -> Integer,
        budget_recurrence_id -> Nullable<Integer>,
    }
}

diesel::table! {
    budget_item_account (account_id, budget_item_id) {
        account_id -> Integer,
        budget_item_id -> Integer,
        allocation_percentage -> Nullable<Integer>,
    }
}

diesel::table! {
    budget_recurrence (id) {
        id -> Integer,
        dt_start -> Integer,
        recurrence_rule -> Text,
        budget_id -> Nullable<Integer>,
        budget_item_id -> Nullable<Integer>,
    }
}

diesel::joinable!(balance -> account (account_id));
diesel::joinable!(budget_item -> budget (budget_id));
diesel::joinable!(budget_item_account -> account (account_id));
diesel::joinable!(budget_item_account -> budget_item (budget_item_id));
diesel::joinable!(budget_recurrence -> budget_item (budget_item_id));
diesel::joinable!(budget_recurrence -> budget (budget_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    balance,
    budget,
    budget_item,
    budget_item_account,
    budget_recurrence,
);
