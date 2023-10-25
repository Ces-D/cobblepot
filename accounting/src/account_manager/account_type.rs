#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl std::string::ToString for AccountType {
    fn to_string(&self) -> String {
        match self {
            AccountType::Asset => "A".to_string(),
            AccountType::Liability => "L".to_string(),
            AccountType::Equity => "E".to_string(),
            AccountType::Revenue => "R".to_string(),
            AccountType::Expense => "X".to_string(),
        }
    }
}
