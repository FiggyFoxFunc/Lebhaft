use std::io::Result;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, 
    Frame, 
    layout::{Constraint, Layout}, 
    style::Stylize, 
    text::Line, 
    widgets::{ListState}
};

use crate::ui::{MainMenu};

pub enum Screen {
    Main
}

pub struct App {
    pub exit: bool,
    current_screen: Screen,
    main_menu: MainMenu<'static>
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            current_screen: Screen::Main,
            main_menu: MainMenu::new()
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

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .constraints([Constraint::Length(1), Constraint::Min(3), Constraint::Length(1)])
            .split(frame.area());

        let title = Line::from("Welcome to Lebhaft!").centered().magenta();
        let title_bottom = Line::from("Your personal music player and organiser").centered().magenta();

        frame.render_widget(title, layout[0]);
        frame.render_stateful_widget(&mut self.main_menu, layout[1], &mut ListState::default());
        frame.render_widget(title_bottom, layout[2]);

    }

    pub fn handle_events(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Esc {
            self.exit = true; 
            return;
        }

        match self.current_screen {
            Screen::Main => {
                let (exit, _state) = self.main_menu.handle_events(key);
                if exit == true {
                    self.exit = exit;
                    return;
                }
            },
        }
    }
}

