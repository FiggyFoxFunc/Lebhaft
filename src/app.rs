use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

use crate::ui::MainMenuWidget;

#[derive(Debug)]
pub enum Screen {
    Main,
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    current_screen: Screen
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            current_screen: Screen::Main
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        while !self.exit {

            terminal.draw(|frame | self.draw(frame)).expect("Unable to draw frame.");
            self.handle_events();
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(MainMenuWidget, frame.area());
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

