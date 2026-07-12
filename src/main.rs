use std::io;

use gti::app::App;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
