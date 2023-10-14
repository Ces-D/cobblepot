use crate::account_manager::account_type::AccountType;

pub enum AccountCause {
    Credit(AccountType),
    Debit(AccountType),
}

pub fn identify_effect(cause: AccountCause) -> AccountEffect {
    match cause {
        AccountCause::Credit(t) => match t {
            AccountType::Asset => AccountEffect::Decrease,
            AccountType::Liability => AccountEffect::Increase,
            AccountType::Equity => AccountEffect::Increase,
            AccountType::Revenue => AccountEffect::Increase,
            AccountType::Expense => AccountEffect::Decrease,
        },
        AccountCause::Debit(t) => match t {
            AccountType::Asset => AccountEffect::Increase,
            AccountType::Liability => AccountEffect::Decrease,
            AccountType::Equity => AccountEffect::Decrease,
            AccountType::Revenue => AccountEffect::Decrease,
            AccountType::Expense => AccountEffect::Increase,
        },
    }
}

#[derive(PartialEq, Eq)]
pub enum AccountEffect {
    Increase,
    Decrease,
}
