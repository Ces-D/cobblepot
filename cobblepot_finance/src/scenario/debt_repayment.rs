#[derive(Debug)]
struct Debt {
    name: String,
    amount: f64,
    interest_rate: f64,
    minimum_monthly_payment: f64,
    payment_due_date: String,
    loan_term: usize, // in months
    monthly_payments: Vec<Payment>,
    extra_payments: Vec<Payment>,
    fees_and_penalties: Vec<FeePenalty>,
}

#[derive(Debug)]
struct Payment {
    date: String,
    amount: f64,
}

#[derive(Debug)]
struct FeePenalty {
    date: String,
    amount: f64,
    description: String,
}

impl Debt {
    // Method to calculate the total amount paid to date
    fn total_paid(&self) -> f64 {
        self.monthly_payments.iter().map(|p| p.amount).sum::<f64>()
            + self.extra_payments.iter().map(|p| p.amount).sum::<f64>()
    }

    // Method to calculate the remaining balance
    fn remaining_balance(&self) -> f64 {
        self.amount - (self.total_paid() - self.total_fees_and_penalties())
    }

    // Method to calculate the total fees and penalties
    fn total_fees_and_penalties(&self) -> f64 {
        self.fees_and_penalties.iter().map(|f| f.amount).sum()
    }

    // Method to add a payment
    fn add_payment(&mut self, date: String, amount: f64, is_extra: bool) {
        if is_extra {
            self.extra_payments.push(Payment { date, amount });
        } else {
            self.monthly_payments.push(Payment { date, amount });
        }
    }

    // Method to add a fee or penalty
    fn add_fee_penalty(&mut self, date: String, amount: f64, description: String) {
        self.fees_and_penalties.push(FeePenalty { date, amount, description });
    }

    // Method to generate a summary of the debt repayment
    fn summary(&self) -> String {
        format!(
            "Debt Repayment Summary:\n\
             Name: {}\n\
             Outstanding Amount: ${:.2}\n\
             Interest Rate: {:.2}%\n\
             Minimum Monthly Payment: ${:.2}\n\
             Payment Due Date: {}\n\
             Loan Term: {} months\n\
             Total Paid: ${:.2}\n\
             Total Fees and Penalties: ${:.2}\n\
             Remaining Balance: ${:.2}\n\
             Payment History: {:?}\n\
             Extra Payments: {:?}\n\
             Fees and Penalties: {:?}",
            self.name,
            self.amount,
            self.interest_rate,
            self.minimum_monthly_payment,
            self.payment_due_date,
            self.loan_term,
            self.total_paid(),
            self.total_fees_and_penalties(),
            self.remaining_balance(),
            self.monthly_payments,
            self.extra_payments,
            self.fees_and_penalties,
        )
    }
}

fn main() {
    // Example usage
    let mut debt = Debt {
        name: String::from("Credit Card"),
        amount: 5000.0,
        interest_rate: 18.0,
        minimum_monthly_payment: 100.0,
        payment_due_date: String::from("2023-07-15"),
        loan_term: 36, // 3 years
        monthly_payments: vec![],
        extra_payments: vec![],
        fees_and_penalties: vec![],
    };

    // Adding a monthly payment
    debt.add_payment(String::from("2023-07-01"), 100.0, false);

    // Adding an extra payment
    debt.add_payment(String::from("2023-07-10"), 50.0, true);

    // Adding a fee
    debt.add_fee_penalty(String::from("2023-07-20"), 25.0, String::from("Late payment fee"));

    println!("{}", debt.summary());
}
