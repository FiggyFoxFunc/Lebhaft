use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, KeyEventState};
use ratatui::{DefaultTerminal, Frame, text::Text};

#[derive(Debug, Default)]
pub struct App {
    exit: bool
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.draw(Self::draw).expect("Failed to draw frame");
            if matches!(
                event::read().expect("Failed to read event"), 
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE
                })
            ) {
                self.exit = true;
                break;
            }
        }
    }

    pub fn draw(frame: &mut Frame) {
        let text = Text::raw("Hello from Lebhaft! <3");
        frame.render_widget(text, frame.area());
    }
}