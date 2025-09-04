use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Flex, Layout}, 
    style::{Style, Stylize}, 
    symbols::border::THICK, 
    text::Line, 
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, StatefulWidget}
};

pub struct MainMenu<'a> {
    items: Vec<ListItem<'a>>,
    state: ListState
}

impl MainMenu<'static> {
    pub fn new() -> Self {
        Self { 
            items: vec![
                ListItem::new(Line::from("Open Player").left_aligned()), 
                ListItem::new(Line::from("Settings").left_aligned()), 
                ListItem::new(Line::from("Exit").left_aligned())
            ], 
            state: ListState::default() 
        }
    }

    // TODO: Handle the selection of an option.
    pub fn handle_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Down => self.state.select_next(),
            KeyCode::Enter => todo!(),
            _ => ()
        }
    }
}

impl StatefulWidget for &mut MainMenu<'static> {
    type State = ListState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, _state: &mut Self::State) {
        // Improve look of the widget.
        let block = Block::bordered()
            .border_set(THICK)
            .title(Line::from("Main Menu").magenta())
            .title_bottom(Line::from("Up: Move Up, Down: Move down, Enter: to select").centered().magenta());

        let [layout] = Layout::vertical([Constraint::Length(area.height/2)]).flex(Flex::Center).areas(area);

        let list = List::new(self.items.clone())
        .block(block)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::new().magenta());

        StatefulWidget::render(list, layout, buf, &mut self.state);
    }
}
