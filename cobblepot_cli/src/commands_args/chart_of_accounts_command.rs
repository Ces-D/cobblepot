use crate::commands_args::chart_account_category::ChartAccountCategory;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::io;

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
