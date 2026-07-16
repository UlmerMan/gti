use ratatui::widgets::{Paragraph, Widget};

use crate::widgets::art;

pub struct Binoculars {
    pub height: u16,
    pub width: u16,
}

impl Binoculars {
    pub fn new() -> Self {
        Self {
            height: art::BINOCULARS.len() as u16,
            width: 32,
        }
    }
}

impl Widget for &Binoculars {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(art::BINOCULARS)
            .alignment(ratatui::layout::HorizontalAlignment::Center)
            .render(area, buf);
    }
}
