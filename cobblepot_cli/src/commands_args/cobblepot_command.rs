use crate::commands_args::chart_of_accounts_command;
use clap::Command;

pub fn create_cobblepot_command_app() -> Command {
    Command::new("cobblepot")
        .about("Personal finance management tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .args_conflicts_with_subcommands(true)
        .subcommand(chart_of_accounts_command::create_command())
}
