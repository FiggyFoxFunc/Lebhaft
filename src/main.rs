mod app;
mod ui;
use app::App;

fn main() {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    app.run(&mut terminal);
    ratatui::restore();
}


