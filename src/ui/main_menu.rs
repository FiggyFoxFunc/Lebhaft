use ratatui::widgets::{ListState, StatefulWidget};

pub struct MainMenuWidget {
    items: Vec<String>,
    state: ListState
}

impl MainMenuWidget {
    pub fn new() -> Self {
        Self { 
            items: vec!["Open Player".into(), "Settings".into(), "Exit".into()], 
            state: ListState::default() 
        }
    }
}

impl StatefulWidget for MainMenuWidget {
    type State = ListState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        todo!();
    }
}
