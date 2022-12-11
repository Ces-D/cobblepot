use super::shared::args::{account_category_arg, account_description_arg, account_name_arg};
use super::shared::traits;

use std::io;

pub struct ChartOfAccountsCommand {
    cmd: clap::Command,
}

impl ChartOfAccountsCommand {
    pub fn new() -> ChartOfAccountsCommand {
        ChartOfAccountsCommand {
            cmd: clap::Command::new("chart_of_accounts")
                .about("Directory for managing accounts")
                .arg_required_else_help(true)
                .short_flag('A')
                .long_flag("chart_of_accounts")
                .subcommand(
                    clap::Command::new("open")
                        .short_flag('O')
                        .long_flag("open")
                        .about("Open a new Account")
                        .after_help("Open an Account in your Chart of Accounts. Provide a name, description, and the account category for this account")
                        .args([account_name_arg(), account_description_arg(), account_category_arg()])
                )
                .subcommand( 
                    clap::Command::new("close")
                        .short_flag('C')
                        .long_flag("close")
                        .about("Close an Account")
                        .after_help("Close an Account in your Chart of Accounts.")
                        .args([account_name_arg()])
                ) 
                .subcommand( 
                    clap::Command::new("save")
                        .short_flag('S')
                        .long_flag("save")
                        .about("Save your changes")
                        .after_help("Save your changes to the Chart of Accounts.")
                )
        }
    }
}

impl traits::CobblepotCommandFactory for ChartOfAccountsCommand {
    fn command(&self) -> clap::Command {
        self.cmd
    }
}

fn out() -> io::Result<()> {
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
            } w
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

// TODO: create a struct that handles the rendering. Maybe it is passed the argMatches and the
// required args options and checks based on it, then renders. Basically I want to create simple
// way of rendering errors, missing pieces, and the rest.
