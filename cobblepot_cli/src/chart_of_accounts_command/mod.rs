pub mod command;

fn out() -> io::Result<()> {
    match command.subcommand() {
        Some(("open", open_matches)) => {
            let name = open_matches.get_one::<String>("name");
            let description = open_matches.get_one::<String>("description");
            let category = open_matches.get_one::<ChartAccountCategory>("category");
            if name.is_none() {
                println!("Missing required value for name");
            }
            if description.is_none() {
                println!("Missing required description");
            } w
            if category.is_none() {
                println!("Missing required account category")
            }
            if name.is_some() && description.is_some() && category.is_some() {
                println!(
                    "You have input: {} {} {}",
                    name.unwrap(),
                    description.unwrap(),
                    category.unwrap()
                );
            }
            Ok(())
        },
        Some(("close", close_matches)) => {
            println!("Close Command was called");
            Ok(())
        },
        Some(("save", save_matches)) => {
            println!("Save Command was called");
            Ok(())
        },
        _ => {
            println!("Nothing was triggered");
            Ok(())
        },
    }
}

// TODO: create a struct that handles the rendering. Maybe it is passed the argMatches and the
// required args options and checks based on it, then renders. Basically I want to create simple
// way of rendering errors, missing pieces, and the rest.
