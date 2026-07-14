use clap::{Command, Arg, ArgAction};

pub fn cli() -> Command {
    Command::new("gti")
        .about("A troll versioning cli")
        .subcommand_required(false)
        .subcommand(
            Command::new("push")
            .arg(
                Arg::new("force")
                    .long("force")
                    .action(ArgAction::SetTrue),
            )
        )
        .subcommand(
            Command::new("pull")
        )
}
