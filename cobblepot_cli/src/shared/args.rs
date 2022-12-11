use super::chart_account_category::ChartAccountCategory;
use clap::{value_parser, Arg, ArgAction};

pub fn account_name_arg() -> Arg {
    Arg::new("name")
        .short('n')
        .long("name")
        .help("Unique name of this account")
        .action(ArgAction::Set)
}

pub fn account_description_arg() -> Arg {
    Arg::new("description")
        .short('d')
        .long("description")
        .help("Description of this account")
        .action(ArgAction::Set)
}

pub fn account_category_arg() -> Arg {
    Arg::new("category")
        .short('c')
        .long("category")
        .help("Type of this account")
        .value_parser(value_parser!(ChartAccountCategory))
        .action(ArgAction::Set)
}
