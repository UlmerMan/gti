use gti::app::App;

fn main() -> color_eyre::Result<()> {
    let matches = gti::cli::cli().get_matches();

    color_eyre::install()?;

    ratatui::run(|terminal| App::from(matches).run(terminal))?;

    Ok(())
}
