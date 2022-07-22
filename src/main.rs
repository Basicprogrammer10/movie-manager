use std::io;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod show;

pub const VERSION: &str = "α1.0";

fn main() {
    show::Show::parse_dir("D:\\Movies");
    // enable_raw_mode().unwrap();
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen).unwrap();
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend).unwrap();

    // let app = app::App::new();
    // app.run(&mut terminal);

    // disable_raw_mode().unwrap();
    // execute!(terminal.backend_mut(), LeaveAlternateScreen,).unwrap();
    // terminal.show_cursor().unwrap();
}
