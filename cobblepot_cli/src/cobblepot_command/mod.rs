use super::chart_of_accounts_command;

pub struct CobblepotCommand {
    cmd: clap::Command,
}

impl CobblepotCommand {
    pub fn new() -> CobblepotCommand {
        let chart_of_accounts_command_instance =
            chart_of_accounts_command::command::ChartOfAccountsCommand {};
        CobblepotCommand {
            cmd: clap::Command::new("cobblepot")
                .about("Personal finance management tool")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .args_conflicts_with_subcommands(true)
                .subcommand(chart_of_accounts_command_instance.command()),
        }
    }

    pub fn command(&self) -> clap::Command {
        self.cmd
    }
}
