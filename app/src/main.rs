mod config;

fn main() {
    let app_config = config::CobblepotConfig::load_config_file();
    app_config.ensure_vault_created();
    // TODO (Cesar) - create the vault in the vault path and the chart of accounts in the vault
    // TODO (Cesar) - create the open_account, close_account, list_accounts, and update_account
    // methods.
    // TODO (Cesar) - create the model for Account; include information to identify the account in
    // human terms
    // TODO (Cesar) - create the moel for an AccountEntry; include information for chart of accounts
    println!("{:?}", app_config);
}
