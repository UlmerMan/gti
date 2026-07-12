use ratatui::widgets::{Paragraph, Widget};

pub struct Car {
    pub height: u16,
    pub width: u16,
    variant: u8,
}

impl Car {
    pub fn set_variant(&mut self, variant: u8) {
        self.variant = variant
    }
    
    pub fn switch_variant(&mut self) {
        if self.variant == 1 {
            self.set_variant(2);
        } else {
            self.set_variant(1);
        }
    }

    pub fn get_height(self) -> u16 {
        self.height
    }

    pub fn get_width(self) -> u16 {
        self.width
    }
}

impl Default for Car {
    fn default() -> Self {
        Self { height: 7, width: 32, variant: 1}
    }
}

impl Widget for &Car {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        const CAR_VARIANT1: &[&str] = &[
            "   ,---------------.",
            "  /  /``````|``````\\\\",
            " /  /_______|_______\\\\________",
            "|]      GTI |'       |        |]",
            "=  .-:-.    |________|  .-:-.  =",
            " `  -+-  --------------  -+-  '",
            "   '-:-'                '-:-'  ",
        ];

        const CAR_VARIANT2: &[&str] = &[
            "   ,---------------.",
            "  /  /``````|``````\\\\",
            " /  /_______|_______\\\\________",
            "|]      GTI |'       |        |]",
            "=  .:-:.    |________|  .:-:.  =",
            " `   X   --------------   X   '",
            "   ':-:'                ':-:'  "
        ];

        match self.variant {
            1 => {return Paragraph::new(CAR_VARIANT1).left_aligned().render(area, buf)},
            2 => {return Paragraph::new(CAR_VARIANT2).left_aligned().render(area, buf)},
            _ => ()
        }
        
    }
}