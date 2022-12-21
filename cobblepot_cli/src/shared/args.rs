use clap::{Arg, ArgAction, ArgMatches};

pub enum AcceptedArg {
    Name,
    Description,
    Category,
}

pub fn create_args(accepted: Vec<AcceptedArg>) -> Vec<Arg> {
    let mut args: Vec<Arg> = Vec::new();

    for ele in accepted {
        let arg = match ele {
            AcceptedArg::Name => Arg::new("name")
                .short('n')
                .help("Unique name of this account")
                .action(ArgAction::Set)
                .required(true),
            AcceptedArg::Description => Arg::new("description")
                .short('d')
                .help("Description of this account")
                .action(ArgAction::Set)
                .required(true),
            AcceptedArg::Category => Arg::new("category")
                .short('c')
                .help("Type of this account")
                .action(ArgAction::Set)
                .required(true),
        };

        args.push(arg);
    }

    args
}

pub fn find_missing_arg(accepted: Vec<AcceptedArg>, matches: ArgMatches) -> Option<AcceptedArg> {}

// set the standard for args and values of args
// maybe this is passed an array of AllowedArgs and it creates the collection of Args for the
// Command
// This can then also get the matches and compare with the identifies allowed args
// struct
// construct - collection of AllowedArgs
//  - internal generate args collection relative to contructor params
//  - takes a ArgMatches and returns the missing values or nothing if all is input - checks for
//  missing
