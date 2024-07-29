#[derive(Debug)]
struct EmergencyFund {
    balance: f64,
    target_amount: f64,
    monthly_expenses: f64,
    income_sources: Vec<String>,
    contributions: Vec<Contribution>,
    withdrawals: Vec<Withdrawal>,
    interest_earned: f64,
    expense_categories: Vec<String>,
    accessibility: String,
    replenishment_plan: String,
}

#[derive(Debug)]
struct Contribution {
    date: String,
    amount: f64,
}

#[derive(Debug)]
struct Withdrawal {
    date: String,
    amount: f64,
    reason: String,
}

impl EmergencyFund {
    // Method to calculate the remaining amount needed to reach the target
    fn remaining_amount(&self) -> f64 {
        self.target_amount - self.balance
    }

    // Method to add a contribution
    fn add_contribution(&mut self, date: String, amount: f64) {
        self.balance += amount;
        self.contributions.push(Contribution { date, amount });
    }

    // Method to add a withdrawal
    fn add_withdrawal(&mut self, date: String, amount: f64, reason: String) {
        self.balance -= amount;
        self.withdrawals.push(Withdrawal { date, amount, reason });
    }

    // Method to generate a summary of the emergency fund
    fn summary(&self) -> String {
        format!(
            "Emergency Fund Summary:\n\
             Current Balance: ${:.2}\n\
             Target Amount: ${:.2}\n\
             Monthly Expenses: ${:.2}\n\
             Remaining Amount Needed: ${:.2}\n\
             Income Sources: {:?}\n\
             Contributions: {:?}\n\
             Withdrawals: {:?}\n\
             Interest Earned: ${:.2}\n\
             Expense Categories: {:?}\n\
             Accessibility: {}\n\
             Replenishment Plan: {}",
            self.balance,
            self.target_amount,
            self.monthly_expenses,
            self.remaining_amount(),
            self.income_sources,
            self.contributions,
            self.withdrawals,
            self.interest_earned,
            self.expense_categories,
            self.accessibility,
            self.replenishment_plan,
        )
    }
}

fn main() {
    // Example usage
    let mut emergency_fund = EmergencyFund {
        balance: 5000.0,
        target_amount: 10000.0,
        monthly_expenses: 2000.0,
        income_sources: vec![String::from("Salary"), String::from("Freelance")],
        contributions: vec![],
        withdrawals: vec![],
        interest_earned: 50.0,
        expense_categories: vec![
            String::from("Medical"),
            String::from("Job Loss"),
            String::from("Home Repairs"),
        ],
        accessibility: String::from("Immediate"),
        replenishment_plan: String::from("Monthly contributions from salary and freelance income."),
    };

    // Adding a contribution
    emergency_fund.add_contribution(String::from("2023-07-01"), 500.0);

    // Adding a withdrawal
    emergency_fund.add_withdrawal(String::from("2023-08-15"), 200.0, String::from("Car repair"));

    println!("{}", emergency_fund.summary());
}
