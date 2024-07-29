#[derive(Debug)]
struct IncomeSource {
    name: String,
    amount: Vec<f64>, // Historical income amounts
    frequency: String,
    stability: String, // e.g., "stable", "variable", "one-time"
}

#[derive(Debug)]
struct IncomeVolatilityRecord {
    income_sources: Vec<IncomeSource>,
    fixed_expenses: f64,
    variable_expenses: f64,
    discretionary_expenses: f64,
    emergency_fund: f64,
    savings_rate: f64,
    debt_obligations: f64,
    buffer_period: f64, // in months
}

impl IncomeVolatilityRecord {
    // Method to calculate total income over a period
    fn total_income(&self) -> f64 {
        self.income_sources.iter().flat_map(|source| source.amount.iter()).sum()
    }

    // Method to calculate average monthly income
    fn average_monthly_income(&self) -> f64 {
        let total_months =
            self.income_sources.iter().flat_map(|source| source.amount.iter()).count();
        if total_months == 0 {
            0.0
        } else {
            self.total_income() / total_months as f64
        }
    }

    // Method to calculate total expenses per month
    fn total_monthly_expenses(&self) -> f64 {
        self.fixed_expenses + self.variable_expenses + self.discretionary_expenses
    }

    // Method to calculate the buffer period based on emergency fund and monthly expenses
    fn calculate_buffer_period(&self) -> f64 {
        if self.total_monthly_expenses() == 0.0 {
            0.0
        } else {
            self.emergency_fund / self.total_monthly_expenses()
        }
    }

    // Method to generate a summary of the income volatility record
    fn summary(&self) -> String {
        format!(
            "Income Volatility Summary:\n\
             Total Income: ${:.2}\n\
             Average Monthly Income: ${:.2}\n\
             Total Monthly Expenses: ${:.2}\n\
             Emergency Fund: ${:.2}\n\
             Buffer Period: {:.2} months\n\
             Savings Rate: {:.2}%\n\
             Debt Obligations: ${:.2}\n\
             Income Sources: {:?}",
            self.total_income(),
            self.average_monthly_income(),
            self.total_monthly_expenses(),
            self.emergency_fund,
            self.calculate_buffer_period(),
            self.savings_rate * 100.0,
            self.debt_obligations,
            self.income_sources,
        )
    }
}
