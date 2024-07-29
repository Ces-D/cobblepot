use rust_decimal::Decimal;

use crate::account::AccountBalance;

use crate::currency::Amount;
use crate::summary::Summary;

/// Uses historical data to determine an average growth rate, which is then applied to project future values.
/// Example: If sales have grown by an average of 5% per year, this rate can be used to forecast future sales.
/// * `current_balance` - starting value for a given period
/// * `previous_balance` - end value over a second time period
pub fn calculate_simple_growth_rate(
    current_balance: AccountBalance,
    previous_balance: AccountBalance,
) -> Summary {
    let debit = (current_balance.debit_balance - previous_balance.debit_balance)
        / previous_balance.debit_balance;
    let credit = (current_balance.credit_balance - previous_balance.credit_balance)
        / previous_balance.credit_balance;

    Summary {
        debits: debit * Amount::from(Decimal::from(100)),
        credits: credit * Amount::from(Decimal::from(100)),
    }
}
