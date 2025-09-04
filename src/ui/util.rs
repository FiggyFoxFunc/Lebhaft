use ratatui::{style::Stylize, symbols::border::THICK, text::Line, widgets::Block};

pub fn build_block(title: String, title_bottom: String) -> Block<'static> {
    let block = Block::bordered()
            .border_set(THICK)
            .title(Line::from(title).magenta())
            .title_bottom(Line::from(title_bottom).centered().magenta());
    block
}