use chrono::{DateTime, TimeDelta, Utc};
use rust_decimal::Decimal;

use crate::account::Frequency;
use crate::currency::Amount;
use crate::historical_record::{Summary, Transfer};

/// Uses historical data to determine an average growth rate, which is then applied to project future values.
/// Example: If sales have grown by an average of 5% per year, this rate can be used to forecast future sales.
/// * `current_balance` - starting value for a given period
/// * `previous_balance` - end value over a second time period
pub fn calculate_simple_growth_rate(
    current_balance: Summary,
    previous_balance: Summary,
) -> Summary {
    let debit = (current_balance.debits - previous_balance.debits) / previous_balance.debits;
    let credit = (current_balance.credits - previous_balance.credits) / previous_balance.credits;

    Summary {
        debits: debit * Amount::from(Decimal::from(100)),
        credits: credit * Amount::from(Decimal::from(100)),
    }
}

/// Uses previous transfer date and the frequency of transfers to calculate the next time a
/// transfer will be made. This is useful when calculating the next time an Expense will charge or
/// an Income will credit to
pub fn next_transfer_date(frequency: Frequency, previous_transfer: DateTime<Utc>) -> DateTime<Utc> {
    match frequency {
        Frequency::Daily(days) => previous_transfer + TimeDelta::days(days.into()),
        Frequency::Weekly(weeks) => previous_transfer + TimeDelta::weeks(weeks.into()),
        Frequency::Monthly(months) => previous_transfer + TimeDelta::weeks((months * 4) as i64),
    }
}
