use gti::{app::App, widgets::car::CarVariants};


fn main() -> color_eyre::Result<()> {
    let matches = gti::cli::cli().get_matches();

    color_eyre::install()?;

    match matches.subcommand() {
        Some(("push", sub_matches)) => {
            match sub_matches.get_one::<String>("force") {
                Some(_) => ratatui::run(|terminal| App::new(CarVariants::Pushing1, true).run(terminal)),
                None => ratatui::run(|terminal| App::new(CarVariants::Pushing1, false).run(terminal))
            }
            
        }
        Some(("pull", _sub_matches)) => {
            ratatui::run(|terminal| App::new(CarVariants::Pulling1, false).run(terminal))
        }
        _ => ratatui::run(|terminal| App::new(CarVariants::Driving1, false).run(terminal)),
    }
}
