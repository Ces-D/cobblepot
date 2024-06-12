use cobblepot_core::error::CobblepotError;
use cobblepot_files::vault::VaultConfig;

mod shared;

#[test]
fn test_add_account_command() {
    let config = shared::create_test_config();
    let cli = cobblepot_cli::create_cli();

    success_add_account(config.clone(), cli.clone());
    failure_multiple_same_accounts(config, cli);
}

fn success_add_account(config: VaultConfig, cli: clap::Command) {
    let matches = cli.get_matches_from(vec![
        "cobblepot",
        "add_account",
        "-n",
        "Test Account",
        "-d",
        "This is a description",
    ]);

    let handled = match matches.subcommand() {
        Some(("add_account", add_account_command_matches)) => {
            cobblepot_cli::add_account_command::command_handler(
                add_account_command_matches,
                &config,
            )
        },
        _ => panic!("Understanding of the arguments are incorrect"),
    }
    .unwrap();

    assert_eq!(handled, ())
}

fn failure_multiple_same_accounts(config: VaultConfig, cli: clap::Command) {
    let matches = cli.get_matches_from(vec![
        "cobblepot",
        "add_account",
        "-n",
        "Test Account",
        "-d",
        "This is a description",
    ]);

    let handled = match matches.subcommand() {
        Some(("add_account", add_account_command_matches)) => {
            cobblepot_cli::add_account_command::command_handler(
                add_account_command_matches,
                &config,
            )
        },
        _ => panic!("Understanding of the arguments are incorrect"),
    };

    match handled {
        Ok(_) => panic!("Test was incorrect"),
        Err(error) => assert_eq!(
            error,
            CobblepotError::AddJournalEntryCliError(
                "Error unable to create chart of accounts account",
            )
        ),
    }
}
