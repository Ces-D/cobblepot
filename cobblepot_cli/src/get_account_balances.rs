use clap::{ArgMatches, Command};
use cobblepot_core::error::CobblepotError;
use cobblepot_files::vault::VaultConfig;

pub fn create_command() -> Command {
    Command::new("get_balances").about("Get all the current balances of your accounts")
}

pub fn command_handler(
    matches: &ArgMatches,
    vault_config: &VaultConfig,
) -> Result<(), CobblepotError> {
    !todo!()
}
