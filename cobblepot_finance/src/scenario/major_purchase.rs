use rust_decimal::Decimal;

use crate::currency::Amount;

#[derive(Debug)]
pub struct MajorPurchase {
    pub purchase_price: Amount,
    pub down_payment: Amount,
    pub loan_amount: Amount,
    pub interest_rate: Decimal,
    pub loan_term: u32, // in months
    pub monthly_payment: Amount,
    pub sales_tax: Amount,
    pub property_tax: Amount,
    pub registration_fees: Amount,
    pub monthly_insurance_cost: Amount,
    pub monthly_maintenance_cost: Amount,
    pub monthly_operating_cost: Amount,
    pub annual_depreciation_rate: Decimal,
    pub resale_value: Amount,
    pub warranty_details: String,
}

impl MajorPurchase {
    pub fn new(
        purchase_price: Amount,
        down_payment: Option<Amount>,
        loan_amount: Option<Amount>,
        interest_rate: Option<Decimal>,
        loan_term_months: Option<u32>,
        monthly_payment: Option<Amount>,
        sales_tax: Option<Amount>,
        property_tax: Option<Amount>,
        registration_fees: Option<Amount>,
        monthly_insurance_cost: Option<Amount>,
        monthly_maintenance_cost: Option<Amount>,
        monthly_operating_cost: Option<Amount>,
        annual_depreciation_rate: Option<Decimal>,
        resale_value: Option<Amount>,
        warranty_details: String,
    ) -> Self {
        MajorPurchase {
            purchase_price,
            down_payment: down_payment.unwrap_or(Amount::from(Decimal::ZERO)),
            loan_amount: loan_amount.unwrap_or(Amount::from(Decimal::ZERO)),
            interest_rate: interest_rate.unwrap_or(Decimal::ZERO),
            loan_term: loan_term_months.unwrap_or(0),
            monthly_payment: monthly_payment.unwrap_or(Amount::from(Decimal::ZERO)),
            sales_tax: sales_tax.unwrap_or(Amount::from(Decimal::ZERO)),
            property_tax: property_tax.unwrap_or(Amount::from(Decimal::ZERO)),
            registration_fees: registration_fees.unwrap_or(Amount::from(Decimal::ZERO)),
            monthly_insurance_cost: monthly_insurance_cost.unwrap_or(Amount::from(Decimal::ZERO)),
            monthly_maintenance_cost: monthly_maintenance_cost
                .unwrap_or(Amount::from(Decimal::ZERO)),
            monthly_operating_cost: monthly_operating_cost.unwrap_or(Amount::from(Decimal::ZERO)),
            annual_depreciation_rate: annual_depreciation_rate.unwrap_or(Decimal::ZERO),
            resale_value: resale_value.unwrap_or(Amount::from(Decimal::ZERO)),
            warranty_details,
        }
    }

    // Method to calculate the total cost of the purchase including taxes and fees
    pub fn total_cost(&self) -> Amount {
        self.purchase_price + self.sales_tax + self.registration_fees
    }

    // Method to calculate the total cost of ownership over the loan term
    pub fn total_cost_of_ownership(&self) -> Amount {
        self.total_cost()
            + (self.monthly_payment * Decimal::from(self.loan_term))
            + (self.monthly_insurance_cost * Decimal::from(self.loan_term) / Decimal::from(12))
            + (self.monthly_maintenance_cost * Decimal::from(self.loan_term) / Decimal::from(12))
            + (self.monthly_operating_cost * Decimal::from(self.loan_term) / Decimal::from(12))
    }

    // Method to calculate the annual depreciation
    pub fn annual_depreciation(&self) -> Amount {
        self.purchase_price * self.annual_depreciation_rate / Decimal::from(100)
    }
}
