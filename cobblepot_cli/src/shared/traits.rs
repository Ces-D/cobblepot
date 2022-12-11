use std::io::Result;

pub trait CobblepotCommandFactory {
    fn command(&self) -> clap::Command;
}

pub trait TerminalOutput {
    fn success_render(&self) -> Result<()>;
    fn error_render(&self) -> Result<()>;
}
