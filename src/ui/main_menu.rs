use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::Line, widgets::{Block, Paragraph, Widget}};

pub struct MainMenuWidget;

impl Widget for MainMenuWidget {
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