use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, KeyEventState};
use ratatui::{text::Text, Frame};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("Failed to draw frame");
        if matches!(
            event::read().expect("Failed to read event"), 
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            })
        ) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello from Lebhaft! <3");
    frame.render_widget(text, frame.area());
}
