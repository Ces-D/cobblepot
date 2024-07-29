#[derive(Debug)]
pub struct MajorPurchase {
    purchase_price: f64,
    down_payment: f64,
    loan_amount: f64,
    interest_rate: f64,
    loan_term: usize, // in months
    monthly_payment: f64,
    sales_tax: f64,
    property_tax: f64,
    registration_fees: f64,
    insurance_cost: f64,
    maintenance_cost: f64,
    operating_cost: f64,
    depreciation_rate: f64,
    resale_value: f64,
    warranty_details: String,
}

impl MajorPurchase {
    // Method to calculate the total cost of the purchase including taxes and fees
    fn total_cost(&self) -> f64 {
        self.purchase_price + self.sales_tax + self.registration_fees
    }

    // Method to calculate the total cost of ownership over the loan term
    fn total_cost_of_ownership(&self) -> f64 {
        self.total_cost()
            + (self.monthly_payment * self.loan_term as f64)
            + (self.insurance_cost * self.loan_term as f64 / 12.0)
            + (self.maintenance_cost * self.loan_term as f64 / 12.0)
            + (self.operating_cost * self.loan_term as f64 / 12.0)
    }

    // Method to calculate the annual depreciation
    fn annual_depreciation(&self) -> f64 {
        self.purchase_price * self.depreciation_rate / 100.0
    }
}
