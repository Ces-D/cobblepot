use crate::shared::args::{find_missing_arg, AcceptedArg};
use crate::shared::{palette, traits};
use clap::ArgMatches;
use std::io::Result;

struct ChartOfAccountsRenderer {
    matches: ArgMatches,
    accepted_args: Vec<AcceptedArg>,
}

impl ChartOfAccountsRenderer {
    pub fn new(accepted_args: Vec<AcceptedArg>, matches: ArgMatches) -> ChartOfAccountsRenderer {
        ChartOfAccountsRenderer { matches, accepted_args }
    }

    fn find_matches_missing_arg(&self) -> Option<AcceptedArg> {
        find_missing_arg(self.accepted_args, self.matches)
    }
}

impl traits::TerminalOutput for ChartOfAccountsRenderer {
    fn success_render(&self) -> Result<()> {
        // TODO: how do we want to render the successes, since this is going to be used for all
        // chart of accounts, we are also going to have to pass the sub command and render success
        // for each one.
        Ok(())
    }

    fn error_render(&self) -> Result<()> {
        let missing = self.find_matches_missing_arg();
        if missing.is_none() {
            return Ok(());
        }

        println!(
            "{} {}",
            palette::Error::heading("Missing Arguments:".to_string()),
            palette::Error::line(missing.unwrap().to_string())
        );

        Ok(())
    }
}
