mod app;
use app::App;

fn main() -> () {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}


