use clap::{
    builder::PossibleValue, value_parser, Arg, ArgAction, ArgMatches, Command, Error, ValueEnum,
};
use cobblepot_core::chart_of_accounts::{Account, AccountCategory, ChartOfAccounts};
use std::io;

// Can only impl traits to values written in this mod. Had to extend by composition of target
// AccountCategory
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ChartAccountCategory {
    AccountCategory(AccountCategory),
}
impl ValueEnum for ChartAccountCategory {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            ChartAccountCategory::AccountCategory(AccountCategory::Asset),
            ChartAccountCategory::AccountCategory(AccountCategory::Equity),
            ChartAccountCategory::AccountCategory(AccountCategory::Expense),
            ChartAccountCategory::AccountCategory(AccountCategory::Revenue),
            ChartAccountCategory::AccountCategory(AccountCategory::Liability),
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            ChartAccountCategory::AccountCategory(AccountCategory::Asset) => {
                PossibleValue::new("Asset").help("An asset")
            },
            ChartAccountCategory::AccountCategory(AccountCategory::Equity) => {
                PossibleValue::new("Equity").help("An equity")
            },
            ChartAccountCategory::AccountCategory(AccountCategory::Expense) => {
                PossibleValue::new("Expense").help("An expense")
            },
            ChartAccountCategory::AccountCategory(AccountCategory::Revenue) => {
                PossibleValue::new("Revenue").help("A revenue")
            },
            ChartAccountCategory::AccountCategory(AccountCategory::Liability) => {
                PossibleValue::new("Liability").help("A liability")
            },
        })
    }
}

impl std::fmt::Display for ChartAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}

impl std::str::FromStr for ChartAccountCategory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}

fn account_name_arg() -> Arg {
    Arg::new("name")
        .short('n')
        .long("name")
        .help("Unique name of this account")
        .action(ArgAction::Set)
}

fn account_description_arg() -> Arg {
    Arg::new("description")
        .short('d')
        .long("description")
        .help("Description of this account")
        .action(ArgAction::Set)
}

fn account_category_arg() -> Arg {
    Arg::new("category")
        .short('c')
        .long("category")
        .help("Type of this account")
        .value_parser(value_parser!(ChartAccountCategory))
        .action(ArgAction::Set)
}

pub fn create_command() -> Command {
    Command::new("chart_of_accounts")
        .about("Directory for managing accounts")
        .arg_required_else_help(true)
        .short_flag('A')
        .long_flag("chart_of_accounts")
        .subcommand(
            Command::new("open")
                .short_flag('O')
                .long_flag("open")
                .about("Open a new Account")
                .after_help("Open an Account in your Chart of Accounts. Provide a name, description, and the account category for this account")
                .args([account_name_arg(), account_description_arg(), account_category_arg()])
        )
        .subcommand( 
            Command::new("close")
                .short_flag('C')
                .long_flag("close")
                .about("Close an Account")
                .after_help("Close an Account in your Chart of Accounts.")
                .args([account_name_arg()])
        ) 
        .subcommand( 
            Command::new("save")
            .short_flag('S')
            .long_flag("save")
            .about("Save your changes")
            .after_help("Save your changes to the Chart of Accounts.")
        )
}

pub fn handle(command: &ArgMatches) -> io::Result<()> {
    match command.subcommand() {
        Some(("open", open_matches)) => {
            let name = open_matches.get_one::<String>("name");
            let description = open_matches.get_one::<String>("description");
            let category = open_matches.get_one::<ChartAccountCategory>("category");
            if name.is_none() {
                println!("Missing required value for name");
            }
            if description.is_none() {
                println!("Missing required description");
            }
            if category.is_none() {
                println!("Missing required account category")
            }
            if name.is_some() && description.is_some() && category.is_some() {
                println!(
                    "You have input: {} {} {}",
                    name.unwrap(),
                    description.unwrap(),
                    category.unwrap()
                );
            }
            Ok(())
        },
        Some(("close", close_matches)) => {
            println!("Close Command was called");
            Ok(())
        },
        Some(("save", save_matches)) => {
            println!("Save Command was called");
            Ok(())
        },
        _ => {
            println!("Nothing was triggered");
            Ok(())
        },
    }
}

// TODO: create a chart of accounts missing error struct to throw when values are missing.
// This error should be caught and force the command or arg to display the error and the print
// help
//
// TODO: start separating this file in sub modules
//
// TODO: There should be functions that handle the core functions of writing and reading from
// store and these functions which should squarely handle the editing and checking of correct
// values entered
