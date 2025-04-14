pub mod query {

    use std::collections::HashMap;

    use chrono::{Datelike, Local};
    use diesel::{
        Connection, ExpressionMethods, QueryResult, RunQueryDsl, query_dsl::methods::FilterDsl,
    };

    use crate::client::{
        model::{AccountDetailed, BalanceDetailed},
        shared::{
            cli::parse_iso8601_variant_datetime,
            report::{
                AccountLevelAnalytics, BehavioralAnalytics, DeepDiveAnalysis, FinancialAnalytics,
                MILESTONE_VALUES,
            },
        },
    };
    fn calculate_account_level_analytics(account: &AccountDetailed) -> AccountLevelAnalytics {
        let opened_on =
            parse_iso8601_variant_datetime(&account.opened_on).expect("Failed to parse date");
        let age_in_days = chrono::Utc::now().naive_utc() - opened_on;
        AccountLevelAnalytics {
            age_in_days: age_in_days.num_days() as i32,
        }
    }

    fn calculate_financial_analytics(entries: &Vec<BalanceDetailed>) -> FinancialAnalytics {
        if entries.is_empty() {
            FinancialAnalytics {
                current_balance: 0.0,
                average_balance: 0.0,
                total_balance_updates: 0,
                historical_max_balance: 0.0,
                historical_min_balance: 0.0,
                past_year_deltas: [None; 12],
                balance_milestone_dates: [None; 14],
            }
        } else {
            // Sorted from oldest to newest
            let mut sorted_entries = entries.clone();
            sorted_entries.sort_by_key(|entry| {
                parse_iso8601_variant_datetime(&entry.entered_on).expect("Unable to parse date")
            });
            let total_balance_updates = sorted_entries.len() as i32;
            let current_balance = sorted_entries.last().unwrap().amount;

            let mut sum = 0.0;
            let mut historical_max_balance = 0.0;
            let mut historical_min_balance = 0.0;
            let cutoff_date = Local::now().naive_local() - chrono::Duration::days(365);
            let mut past_year_deltas = [None; 12];
            let mut balance_milestone_dates: [Option<chrono::NaiveDateTime>; 14] = [None; 14];

            // Initially, assume the account is below every milestone.
            let mut was_below = [true; 14];
            // Track the balance for each month in the past year.
            let mut monthly_balances: HashMap<u32, f32> = HashMap::new();

            for entry in sorted_entries.iter() {
                let entered_on = parse_iso8601_variant_datetime(&entry.entered_on).unwrap();
                if entry.amount > historical_max_balance {
                    historical_max_balance = entry.amount;
                }
                if entry.amount < historical_min_balance {
                    historical_min_balance = entry.amount;
                }

                if entered_on >= cutoff_date {
                    let month = entered_on.month() - 1;
                    monthly_balances.insert(month, entry.amount);
                }

                for (i, &threshold) in MILESTONE_VALUES.iter().enumerate() {
                    // Only record a crossing if the previous state was below the threshold.
                    if was_below[i] && entry.amount >= threshold {
                        balance_milestone_dates[i] = Some(entered_on);
                        was_below[i] = false; // Now the milestone is considered "active"
                    } else if entry.amount < threshold {
                        // If the balance drops below the threshold, we reset the state.
                        was_below[i] = true;
                    }
                }

                sum += entry.amount;
            }

            for (m, b) in monthly_balances.iter() {
                if m == &0 {
                    past_year_deltas[0] = Some(*b);
                } else {
                    match monthly_balances.get(&(m - 1)) {
                        Some(previous_b) => {
                            let delta = *b - previous_b;
                            past_year_deltas[*m as usize] = Some(delta);
                        }
                        None => {
                            // If there is no previous balance, we can't calculate a delta.
                            past_year_deltas[*m as usize] = None;
                        }
                    }
                }
            }

            let average_balance = sum / total_balance_updates as f32;

            FinancialAnalytics {
                current_balance,
                average_balance,
                total_balance_updates,
                historical_max_balance,
                historical_min_balance,
                past_year_deltas,
                balance_milestone_dates,
            }
        }
    }

    fn calculate_behavioral_analytics(entries: &Vec<BalanceDetailed>) -> BehavioralAnalytics {
        if entries.is_empty() {
            BehavioralAnalytics {
                days_since_last_transaction: 0,
            }
        } else {
            let mut sorted_entries = entries.clone();
            sorted_entries.sort_by_key(|entry| {
                parse_iso8601_variant_datetime(&entry.entered_on).expect("Unable to parse date")
            });

            let today = Local::now().naive_local();
            let last_entered_on =
                parse_iso8601_variant_datetime(&sorted_entries.last().unwrap().entered_on).unwrap();
            let days_since_last_transaction = today - last_entered_on;
            BehavioralAnalytics {
                days_since_last_transaction: days_since_last_transaction.num_days() as i32,
            }
        }
    }

    pub fn get_analytics(
        account_id: i32,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<DeepDiveAnalysis> {
        use crate::schema::{
            account::dsl::{account, id as account_table_id},
            balance::dsl::{account_id as balance_account_id, balance},
        };

        connection.transaction(|conn| {
            let account_detailed = account
                .filter(account_table_id.eq(account_id))
                .first::<AccountDetailed>(conn)?;
            let account_balances_detailed = balance
                .filter(balance_account_id.eq(account_id))
                .load::<BalanceDetailed>(conn)?;

            Ok(DeepDiveAnalysis {
                account: account_detailed.clone(),
                account_level: calculate_account_level_analytics(&account_detailed),
                financial: calculate_financial_analytics(&account_balances_detailed),
                behavioral: calculate_behavioral_analytics(&account_balances_detailed),
            })
        })
    }
}
