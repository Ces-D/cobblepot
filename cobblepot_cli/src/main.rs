mod add_journal_entry_command;
mod arguments;
mod utils;

fn main() {
    let cli = clap::Command::new("cobblepot")
        .about("Personal use accounting cli")
        .subcommand_required(true)
        .subcommand(add_journal_entry_command::create_command());

    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("add_journal_entry", add_journal_entry_command_matches)) => {},
        _ => {
            panic!("Unknown subcommand")
        },
    }
}
