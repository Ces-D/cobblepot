use clap::{Arg, ArgAction, ArgMatches};

#[derive(PartialEq, strum_macros::Display, strum_macros::IntoStaticStr)]
pub enum AcceptedArg {
    Name,
    Description,
    Category,
}

pub fn create_args(accepted: Vec<AcceptedArg>) -> Vec<Arg> {
    let mut args: Vec<Arg> = Vec::new();

    let name: &'static str = AcceptedArg::Name.into();
    let description: &'static str = AcceptedArg::Description.into();
    let category: &'static str = AcceptedArg::Category.into();

    for ele in accepted {
        let arg = match ele {
            AcceptedArg::Name => Arg::new(name)
                .short('n')
                .help("Unique name of this account")
                .action(ArgAction::Set)
                .required(true),
            AcceptedArg::Description => Arg::new(description)
                .short('d')
                .help("Description of this account")
                .action(ArgAction::Set)
                .required(true),
            AcceptedArg::Category => Arg::new(category)
                .short('c')
                .help("Type of this account")
                .action(ArgAction::Set)
                .required(true),
        };

        args.push(arg);
    }

    args
}

pub fn find_missing_arg(accepted: Vec<AcceptedArg>, matches: ArgMatches) -> Option<AcceptedArg> {
    if accepted.contains(&AcceptedArg::Name) && !matches.contains_id("name") {
        return Option::Some(AcceptedArg::Name);
    }
    if accepted.contains(&AcceptedArg::Description) && !matches.contains_id("description") {
        return Option::Some(AcceptedArg::Description);
    }
    if accepted.contains(&AcceptedArg::Category) && !matches.contains_id("category") {
        return Option::Some(AcceptedArg::Category);
    }

    Option::None
}
