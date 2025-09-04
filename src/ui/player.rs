use ratatui::{style::Stylize, symbols::border::THICK, text::Line, widgets::{Block, Paragraph, Widget}};

pub struct Player {

}

impl Player {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Widget for &mut Player  {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::bordered()
            .border_set(THICK)
            .title(Line::from("Player").magenta())
            .title_bottom(Line::from("Intructions").centered().magenta());

        Paragraph::new(Line::from("To be done.")).block(block).centered().render(area, buf);
    }
}