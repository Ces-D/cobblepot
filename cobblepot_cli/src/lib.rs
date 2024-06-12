pub mod add_account_command;
pub mod add_journal_entry_command;
mod arguments;
pub mod get_account_balances;
mod parsers;

pub fn create_cli() -> clap::Command {
    clap::Command::new("cobblepot")
        .about("Personal use accounting cli")
        .subcommand_required(true)
        .subcommand(add_journal_entry_command::create_command())
        .subcommand(add_account_command::create_command())
        .subcommand(get_account_balances::create_command())
}

pub fn run() {
    match cobblepot_files::vault::read_vault_config() {
        Ok(config) => {
            let cli = create_cli();
            let matches = cli.get_matches();

            // TODO: handle all potential errors in the handlers
            let handled = match matches.subcommand() {
                Some(("add_journal_entry", add_journal_entry_command_matches)) => {
                    add_journal_entry_command::command_handler(
                        add_journal_entry_command_matches,
                        &config,
                    )
                },
                Some(("add_account", add_account_command_matches)) => {
                    add_account_command::command_handler(add_account_command_matches, &config)
                },
                Some(("get_balances", get_account_balances_matches)) => {
                    get_account_balances::command_handler(&get_account_balances_matches, &config)
                },
                _ => Ok(()),
            };

            if handled.is_err() {
                println!("{}", handled.expect_err("Should be error"))
            }
        },
        Err(e) => {
            println!("Error creating vault: {}", e);
        },
    }
}
