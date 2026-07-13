use clap::Command;

pub fn cli() -> Command {
    Command::new("gti")
        .about("A troll versioning cli")
        .subcommand_required(false)
        .subcommand(
            Command::new("push")
        )
        .subcommand(
            Command::new("pull")
        )
}
