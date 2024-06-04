mod add_account_command;
mod add_journal_entry_command;
mod arguments;
mod get_account_balances;
mod parsers;

// TODO: consider if the entire cli should actually be a tui app
fn main() {
    match cobblepot_files::vault::read_vault_config() {
        Ok(config) => {
            let cli = clap::Command::new("cobblepot")
                .about("Personal use accounting cli")
                .subcommand_required(true)
                .subcommand(add_journal_entry_command::create_command())
                .subcommand(add_account_command::create_command());

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
