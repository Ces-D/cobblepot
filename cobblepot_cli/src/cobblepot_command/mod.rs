use super::shared::traits;

pub struct CobblepotCommand {
    cmd: clap::Command,
}

impl CobblepotCommand {
    pub fn new() -> CobblepotCommand {
        CobblepotCommand {
            cmd: clap::Command::new("cobblepot")
                .about("Personal finance management tool")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .args_conflicts_with_subcommands(true)
                .subcommand(chart_of_accounts::create()),
        }
    }
}

impl traits::CobblepotCommandFactory for CobblepotCommand {
    fn command(&self) -> clap::Command {
        self.cmd
    }
}
