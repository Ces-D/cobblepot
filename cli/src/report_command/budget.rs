use crate::{
    inform,
    logger::table::{ColumnConfig, Table},
    report_command::dto::{BudgetData, LatestBalanceRow},
    shared::format_money_usd,
    success,
};
use cobblepot_data_store::{RecurrenceRule, UnixTimestamp};
use rrule::{RRuleResult, RRuleSet};
use std::collections::HashMap;

#[derive(Debug)]
struct BudgetItem {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub amount: f32,
}

#[derive(Debug)]
struct BudgetRecurrence {
    pub id: i32,
    pub dt_start: UnixTimestamp,
    pub recurrence_rule: RecurrenceRule,
}

#[derive(Debug)]
pub struct BudgetReport {
    id: i32,
    name: String,
    description: Option<String>,
    anticipated_amount: f32,
    recurrence: Option<(i32, UnixTimestamp, RecurrenceRule)>,
    budget_items: HashMap<i32, BudgetItem>,
    /// HashMap<budget_item_id, BudgetRecurrence>
    budget_item_recurrences: HashMap<i32, BudgetRecurrence>,
    /// HashMap<account_id, Vec<(budget_item_id, allocation_percentage)>>
    budget_item_accounts: HashMap<i32, Vec<(i32, i32)>>,
    account_latest_balances: HashMap<i32, LatestBalanceRow>,
}

impl BudgetReport {
    pub fn from_data(data: BudgetData) -> Self {
        let (
            (budget, budget_recurrence_id, budget_recurrence_dt_start, budget_recurrence_rule),
            budget_items_with_allocations,
        ) = data;
        let recurrence =
            zip_all(budget_recurrence_id, budget_recurrence_dt_start, budget_recurrence_rule);
        let mut budget_items: HashMap<i32, BudgetItem> = HashMap::new();
        let mut budget_item_recurrences: HashMap<i32, BudgetRecurrence> = HashMap::new();
        let mut budget_item_accounts: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

        for bia in budget_items_with_allocations.iter() {
            let bi = BudgetItem {
                id: bia.item_id,
                name: bia.item_name.clone(),
                description: bia.item_description.clone(),
                amount: bia.item_amount,
            };
            budget_items.insert(bia.item_id, bi);
            if let Some((id, dt_start, recurrence_rule)) = zip_all(
                bia.item_recurrence_id,
                bia.item_recurrence_dt_start,
                bia.item_recurrence_rule.clone(),
            ) {
                if !budget_item_recurrences.contains_key(&bia.item_id) {
                    budget_item_recurrences.insert(
                        bia.item_id,
                        BudgetRecurrence {
                            id,
                            dt_start,
                            recurrence_rule,
                        },
                    );
                }
            };
            if let Some((account_id, allocation_percent)) =
                bia.account_id.zip(bia.allocation_percentage)
            {
                if let Some(old) = budget_item_accounts.get_mut(&account_id) {
                    old.push((bia.item_id, allocation_percent));
                } else {
                    budget_item_accounts
                        .insert(account_id, vec![(bia.item_id, allocation_percent)]);
                }
            }
        }

        Self {
            id: budget.id,
            name: budget.name,
            description: budget.description,
            anticipated_amount: budget.anticipated_amount,
            recurrence,
            budget_items,
            budget_item_recurrences,
            budget_item_accounts,
            account_latest_balances: HashMap::new(),
        }
    }

    pub fn set_account_latest_balances(&mut self, b: Vec<LatestBalanceRow>) {
        for balance in b {
            self.account_latest_balances.insert(balance.account_id, balance);
        }
    }

    fn display_budget(&self) {
        inform!("Budget Report:", &self.name);
        if let Some(desc) = &self.description {
            inform!("Description:", desc);
        }
        inform!("Anticipated Amount:", format_money_usd(self.anticipated_amount));
        if let Some((_, dt_start, rule)) = &self.recurrence {
            display_recurrence_rule(dt_start, rule, self.anticipated_amount);
        };
    }

    fn display_budget_items(&self) -> f32 {
        success!("Budget items");
        let mut total = 0.0;
        for bi in self.budget_items.values() {
            total += bi.amount;
            inform!(bi.name, bi.description.clone().unwrap_or_else(|| "".to_string()));
            println!("   Amount: {:>39}", format_money_usd(bi.amount));
            if let Some(recurring) = self.budget_item_recurrences.get(&bi.id) {
                display_recurrence_rule(&recurring.dt_start, &recurring.recurrence_rule, bi.amount);
            }
        }
        total
    }

    fn display_budget_item_account(&self) {
        let supporting_account_ids = self.budget_item_accounts.keys();
        if supporting_account_ids.len() == 0 {
            return;
        }
        success!("Involved Accounts");
        for account_id in supporting_account_ids {
            if let Some(account_balance) = self.account_latest_balances.get(account_id) {
                inform!(
                    format!("Items depending on account: {}", account_balance.account_id),
                    account_balance.account_name
                );
                if let Some(budget_dependencies) = self.budget_item_accounts.get(account_id) {
                    let mut dependency_total_amount = 0.0;
                    for dependency in budget_dependencies {
                        if let Some(item) = self.budget_items.get(&dependency.0) {
                            let amount_covered = item.amount * (dependency.1 as f32 / 100.0);
                            dependency_total_amount += amount_covered;
                            println!(
                                "   {:<5} {:<31} {}",
                                item.id,
                                item.name,
                                format_money_usd(amount_covered)
                            );
                        }
                    }
                    println!(
                        "   Account Balance remaining:            {}",
                        format_money_usd(account_balance.amount - dependency_total_amount)
                    );
                }
            }
        }
    }

    pub fn display(&self) {
        self.display_budget();
        println!();
        println!();
        let total_amount = self.display_budget_items();
        println!();
        println!();
        self.display_budget_item_account();
        println!();
        println!();
        inform!("Total Amount", format_money_usd(total_amount));
    }
}

fn zip_all<A, B, C>(a: Option<A>, b: Option<B>, c: Option<C>) -> Option<(A, B, C)> {
    match a.zip(b) {
        Some(e) => match c {
            Some(r) => Some((e.0, e.1, r)),
            None => None,
        },
        None => None,
    }
}

fn get_next_occurences(
    rule: rrule::RRule<rrule::Validated>,
    dt_start: &UnixTimestamp,
) -> RRuleResult {
    log::trace!("Starting next occurences {}", rule.to_string());
    let temp_set =
        RRuleSet::new(dt_start.inner().and_utc().with_timezone(&rrule::Tz::UTC)).rrule(rule);
    // let temp_set = RRuleSet::from_str(rule.to_string().as_str()).unwrap();
    let after_now = temp_set.after(chrono::Utc::now().with_timezone(&rrule::Tz::UTC));
    after_now.all(10)
}

fn display_recurrence_rule(dt_start: &UnixTimestamp, rule: &RecurrenceRule, amount: f32) {
    match rule.clone().validate(dt_start.inner().and_utc().with_timezone(&rrule::Tz::UTC)) {
        Ok(validated) => {
            let next_occurences = get_next_occurences(validated.clone(), dt_start);
            if next_occurences.dates.len() > 0 {
                println!(
                    "   Next {} Occurrences Total: {:>21}",
                    next_occurences.dates.len(),
                    format_money_usd(next_occurences.dates.len() as f32 * amount)
                );
                let mut next = "   ".to_string();
                for (index, dt) in next_occurences.dates.iter().enumerate() {
                    let iter = format!(
                        "{} -> {:<15}",
                        &dt.format("%Y-%m-%d").to_string(),
                        format_money_usd((index as f32 + 1.0) * amount)
                    );
                    next.push_str(&iter);
                }
                println!("{}", next);
            }
        }
        Err(e) => log::error!("Error validating recurrence: {}", e),
    };
}
