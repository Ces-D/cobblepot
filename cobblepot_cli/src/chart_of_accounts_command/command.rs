use crate::shared::args::{create_args, AcceptedArg};
use crate::shared::traits;
use clap::Command;

pub struct ChartOfAccountsCommand {}

impl ChartOfAccountsCommand {}

impl traits::CobblepotCommandFactory for ChartOfAccountsCommand {
    fn command(&self) -> Command {
        let main_cmd = Command::new("chart_of_accounts")
            .about("Directory for managing accounts")
            .arg_required_else_help(true)
            .short_flag('A')
            .long_flag("chart_of_accounts");

        let open_sub = Command::new("open")
                        .short_flag('O')
                        .long_flag("open")
                        .about("Open a new Account")
                        .after_help("Open an Account in your Chart of Accounts. Provide a name, description, and the account category for this account")
            .args(create_args(vec![AcceptedArg::Name, AcceptedArg::Description, AcceptedArg::Category]));

        let close_sub = Command::new("close")
            .short_flag('C')
            .long_flag("close")
            .about("Close an Account")
            .after_help("Close an Account in your Chart of Accounts.")
            .args(create_args(vec![AcceptedArg::Name]));

        let save_sub = Command::new("save")
            .short_flag('S')
            .long_flag("save")
            .about("Save your changes")
            .after_help("Save your changes to the Chart of Accounts.");

        main_cmd.subcommand(open_sub).subcommand(close_sub).subcommand(save_sub)
    }
}
