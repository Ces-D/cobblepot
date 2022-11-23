use clap::{Arg, Command, CommandFactory};

struct ChartOfAccountsCommandFactory;

impl CommandFactory for ChartOfAccountsCommandFactory {
    fn command() -> Command {
        Command::new("open")
            .about("Open a new Account")
            .after_help("Open an Account in your Chart of Accounts. Provide a name, description, and the account category for this account")
    }
    fn command_for_update() -> Command {
        Command::new("open")
    }
}

// TOOD: finish these command
// create commands for open, close, save
