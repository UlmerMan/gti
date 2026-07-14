use clap::{Command, arg};

pub fn cli() -> Command {
    Command::new("gti")
        .about("A troll versioning cli")
        .subcommand_required(false)
        .subcommand(
            Command::new("push")
            .arg(arg!(--force))
        )
        .subcommand(
            Command::new("pull")
        )
}
