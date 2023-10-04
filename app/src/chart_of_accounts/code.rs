use crate::chart_of_accounts::enums::AccountType;

/// The numbering system used in a chart of accounts should be logical and consistent, making it easy to add new accounts as needed and to generate reports and financial statements.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct AccountCode {
    // The first digit of an account number indicates the account type.
    account_type: AccountType,

   // TODO (Cesar): think about how to include divisions in the account code. 

    // The last digit is used to identify the specific index id of this account in its
    // account-subaccounts section.
    index: i32,
}

// TODO: Implement a way to generate account codes.
