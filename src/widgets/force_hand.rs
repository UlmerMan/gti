use ratatui::layout::Position;
use ratatui::widgets::Paragraph;

use rand::RngExt;
use ratatui::{
    style::{Color, Style},
    widgets::Widget,
};

use crate::widgets::art;

pub struct ForceHand {
    pub height: u16,
    pub width: u16,
}

impl Widget for &ForceHand {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        Paragraph::new(art::FORCE_HAND)
            .left_aligned()
            .render(area, buf);

        let mut rng = rand::rng();

        let start_x = rng.random_range(0..area.width as i32);
        let start_y = rng.random_range(0..area.height as i32);

        let dx = rng.random_range(-1.0..1.0);
        let dy = rng.random_range(-1.0..1.0);

        let len = rng.random_range(10..40);

        let mut x = start_x as f32;
        let mut y = start_y as f32;

        for _ in 0..len {
            if x >= 0.0 && y >= 0.0 && x < area.width as f32 && y < area.height as f32 {
                let cell = buf
                    .cell_mut(Position::new(area.x + x as u16, area.y + y as u16))
                    .unwrap();

                cell.set_char('*');
                cell.set_style(Style::default().fg(Color::Cyan));
            }

            x += dx;
            y += dy;
        }
    }
}

impl Default for ForceHand {
    fn default() -> Self {
        Self {
            height: 9,
            width: 30,
        }
    }
}
