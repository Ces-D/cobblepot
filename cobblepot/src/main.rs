mod account;
mod apply;
mod balance;
mod cli;
mod infrastructure;
mod recurring_transation;
mod report;
mod schema;
mod shared;

use std::{io::Write, str::FromStr};

pub fn main() {
    let config = infrastructure::config::Config::open();
    let connection = config.establish_connection();

    let app = cli::command();
    let m = app.get_matches();

    let (action_str, action_mtches) = m.subcommand().expect("Action subcmd required.");
    let (noun_str, noun_mtches) = action_mtches.subcommand().expect("Noun subcmd required");

    let data: &String = noun_mtches.get_one("data").expect("Data argument is required");
    let action = cli::CobblepotCommand::from_str(action_str);
    let noun = cli::CobblepotCommand::from_str(noun_str);

    assert!(action.is_ok() && noun.is_ok(), "Failure to parse either action or noun subcommand");

    let action = action.unwrap();
    let noun = noun.unwrap();

    match cli::handle(action, noun, connection, data) {
        Ok(json_serializable) => {
            let mut writer = std::io::BufWriter::new(std::io::stdout());
            writer.write_all(json_serializable.as_bytes()).unwrap();
        }
        Err(error) => match error {
            shared::CobblepotError::DieselError(error) => eprintln!("{}", error.to_string()),
            shared::CobblepotError::JSONSerializationError(error) => {
                eprintln!("{}", error.to_string())
            }
            shared::CobblepotError::CliCommandError(error) => eprintln!("{}", error.to_string()),
            shared::CobblepotError::LogicError(error) => eprintln!("{}", error),
            shared::CobblepotError::RruleError(error) => eprintln!("{}", error.to_string()),
        },
    }
}
