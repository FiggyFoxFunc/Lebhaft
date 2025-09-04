use ratatui::{text::Line, widgets::{Paragraph, Widget}};

use crate::ui::util;

pub struct Player {

}

impl Player {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Widget for &mut Player  {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = util::build_block(String::from("Player"), String::from("Instructions"));
        Paragraph::new(Line::from("To be done.")).block(block).centered().render(area, buf);
    }
}