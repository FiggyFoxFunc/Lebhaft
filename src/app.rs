use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::Rect, 
    style::Stylize, symbols::border, text::Line, 
    widgets::{Block, Paragraph, Widget}
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool

}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        while !self.exit {

            terminal.draw(|frame | self.draw(frame)).expect("Unable to draw frame.");
            self.handle_events();
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(AppWidget, frame.area());
    }

    pub fn handle_events(&mut self) {
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
        }
    }
}

struct AppWidget;

impl Widget for AppWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Welcome to Lebhaft!").centered().magenta();
        let title_bottom = Line::from("Your personal music player and organiser").centered().magenta();
        let block = Block::bordered()
            .title(title)
            .title_bottom(title_bottom)
            .border_set(border::THICK);
        let _ = Paragraph::new("Still a work in progress...").centered().block(block).render(area, buf);
    }   
}