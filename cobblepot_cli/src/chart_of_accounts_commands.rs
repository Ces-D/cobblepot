use clap::{Arg, Command};

fn create_commands() -> Vec<Command> {
    let open_command = Command::new("open")
        .about("Open a new Account")
        .after_help("Open an Account in your Chart of Accounts. Provide a name, description, and the account category for this account");

    let close_command = Command::new("close")
        .about("Close an Account")
        .after_help("Close an Account in your Chart of Accounts.");

    let save_command = Command::new("save")
        .about("Save your changes")
        .after_help("Save your changes to the Chart of Accounts.");
    vec![open_command, close_command, save_command]
}
