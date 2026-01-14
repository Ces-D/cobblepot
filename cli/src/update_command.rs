mod dto;

use crate::{alert, success};
use clap::{Parser, Subcommand};
use cobblepot_data_store::{RecurrenceRule, UnixTimestamp};
use diesel::SqliteConnection;

#[derive(Debug, Subcommand)]
pub enum UpdateCommand {
    #[clap(about = "Update a budget")]
    Budget {
        #[clap(help = "Id of the budget")]
        id: i32,
        #[clap(short, long, help = "Name of the budget")]
        name: Option<String>,
        #[clap(short, long, help = "Description of the budget")]
        description: Option<String>,
        #[clap(short, long, help = "Expected dollar limit")]
        anticipated_amount: Option<f32>,
        #[clap(short='s', long, value_parser = crate::shared::parse_date, help = "Date budget recurrence starts (format: YYYY-MM-DD)")]
        r_start: Option<chrono::NaiveDateTime>,
        #[clap(
            short,
            long,
            help = "Recurrence rule signaling repeating budget. Must pair with r_start if new"
        )]
        r_rule: Option<String>,
    },
    #[clap(about = "Update a budget line item")]
    BudgetItem {
        #[clap(help = "Id of the budget item")]
        id: i32,
        #[clap(short, long, help = "Id of the budget")]
        budget_id: Option<i32>,
        #[clap(short, long, help = "Name of the line item")]
        name: Option<String>,
        #[clap(short, long, help = "Description of the line item")]
        description: Option<String>,
        #[clap(short, long, help = "Value of this item")]
        amount: Option<f32>,
        #[clap(short='s', long, value_parser = crate::shared::parse_date, help = "Date budget recurrence starts (format: YYYY-MM-DD)")]
        r_start: Option<chrono::NaiveDateTime>,
        #[clap(
            short,
            long,
            help = "Recurrence rule signaling repeating budget item. Must pair with r_start if new"
        )]
        r_rule: Option<String>,
    },
}

#[derive(Debug, Parser)]
pub struct UpdateArgs {
    #[clap(subcommand)]
    pub command: UpdateCommand,
}

pub fn handle_update_command(args: UpdateArgs, conn: SqliteConnection) {
    match args.command {
        UpdateCommand::Budget {
            id,
            name,
            description,
            anticipated_amount,
            r_start,
            r_rule,
        } => {
            if r_rule.is_some() || r_start.is_some() {
                alert!("Validating recurrence rules");
            };
            match dto::update_budget(
                conn,
                id,
                name,
                description,
                anticipated_amount,
                r_start.map(|v| UnixTimestamp::new(v)),
                r_rule.map(|v| RecurrenceRule::new(v)),
            ) {
                Ok(res) => success!("Updated budget: {} - {}", res.0, res.1),
                Err(e) => {
                    alert!("Failed to update budget");
                    log::error!("Update Budget: {}", e);
                }
            }
        }
        UpdateCommand::BudgetItem {
            id,
            budget_id,
            name,
            description,
            amount,
            r_start,
            r_rule,
        } => {
            if r_rule.is_some() || r_start.is_some() {
                alert!("Validating recurrence rules");
            };
            match dto::update_budget_item(
                conn,
                id,
                budget_id,
                name,
                description,
                amount,
                r_start.map(|v| UnixTimestamp::new(v)),
                r_rule.map(|v| RecurrenceRule::new(v)),
            ) {
                Ok(res) => success!("Updated budget item: {} - {}", res.0, res.1),
                Err(e) => {
                    alert!("Failed to update budget");
                    log::error!("Update Budget Item: {}", e);
                }
            }
        }
    }
}
