use rusty_money::iso::Currency;
use rusty_money::Money;

pub struct Entry {
    amount: Money<'static, Currency>,
    memo: String,
}

impl Entry {
    pub fn new(amount: Money<'static, Currency>, memo: String) -> Entry {
        Entry { amount, memo }
    }

    pub fn update_amount(&mut self, amount: Money<'static, Currency>) {
        self.amount = amount;
    }

    pub fn update_memo(&mut self, memo: String) {
        self.memo = memo;
    }

    pub fn read_amount(&self) -> Money<'static, Currency> {
        self.amount.clone()
    }

    pub fn read_memo(&self) -> String {
        self.memo.clone()
    }
}
