use std::io::Result;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, Frame, style::Stylize, symbols::border, text::Line, widgets::Block};

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

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {

            terminal.draw(|frame | self.draw(frame)).expect("Unable to draw frame.");
            if let Some(key) = crossterm::event::read()?.as_key_press_event() {
                self.handle_events(key);
            }
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        let title = Line::from("Welcome to Lebhaft!").centered().magenta();
        let title_bottom = Line::from("Your personal music player and organiser").centered().magenta();
        let block = Block::bordered()
            .title(title)
            .title_bottom(title_bottom)
            .border_set(border::THICK);

        frame.render_widget(block, frame.area());
    }

    pub fn handle_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.exit = true,
            _ => ()
        }
    }
}

