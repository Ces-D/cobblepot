use crate::account::account_type::AccountType;

pub struct AccountCode(String);

impl AccountCode {
    pub fn new(account_type: AccountType, index: i32) -> AccountCode {
        AccountCode(format!("{}{}", account_type.to_string(), index))
    }

    pub fn get_account_code(&self) -> &String {
        &self.0
    }

    pub fn extract_account_type(&self) -> AccountType {
        match &self.0[0..1] {
            "A" => AccountType::Asset,
            "L" => AccountType::Liability,
            "E" => AccountType::Equity,
            "R" => AccountType::Revenue,
            "X" => AccountType::Expense,
            _ => panic!("Invalid account type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_code() {
        let account_code = AccountCode::new(AccountType::Asset, 1);
        assert_eq!("A1", account_code.get_account_code());
        assert_eq!(AccountType::Asset, account_code.extract_account_type());
    }
}
