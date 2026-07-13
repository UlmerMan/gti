use std::io;

use gti::{app::App, widgets::car::CarVariants};

fn main() -> io::Result<()> {
    let matches = gti::cli::cli().get_matches();

    match matches.subcommand() {
        Some(("push", _sub_matches)) => {
            ratatui::run(|terminal| App::new(CarVariants::Pushing1).run(terminal))
        }
        Some(("pull", _sub_matches)) => {
            ratatui::run(|terminal| App::new(CarVariants::Pulling1).run(terminal))
        }
        _ => ratatui::run(|terminal| App::new(CarVariants::Driving1).run(terminal)),
    }
}
