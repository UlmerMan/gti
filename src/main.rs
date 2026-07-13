use gti::{app::App, widgets::car::CarVariants};


fn main() -> color_eyre::Result<()> {
    let matches = gti::cli::cli().get_matches();

    color_eyre::install()?;

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
