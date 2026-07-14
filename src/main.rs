use gti::{app::App, widgets::car::CarVariants};


fn main() -> color_eyre::Result<()> {
    let matches = gti::cli::cli().get_matches();

    color_eyre::install()?;

    match matches.subcommand() {
        Some(("push", sub_matches)) => {
            if sub_matches.get_flag("force") {
                ratatui::run(|terminal| App::new(CarVariants::Driving1, true).run(terminal))
            } else {
                ratatui::run(|terminal| App::new(CarVariants::Pushing1, false).run(terminal))
            }
        }
        Some(("pull", _sub_matches)) => {
            ratatui::run(|terminal| App::new(CarVariants::Pulling1, false).run(terminal))
        }
        _ => ratatui::run(|terminal| App::new(CarVariants::Driving1, false).run(terminal)),
    }
}
