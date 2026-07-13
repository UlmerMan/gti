use ratatui::widgets::{Paragraph, Widget};

use crate::widgets::art;

pub enum CarVariants {
    Driving1,
    Driving2,
    Pushing1,
    Pushing2,
    Pulling1,
    Pulling2,
}

pub struct Car {
    pub height: u16,
    pub width: u16,
    variant: CarVariants,
}

impl Car {
    pub fn new(variant: CarVariants) -> Self {
        let mut car = Self::default();
        car.set_variant(variant);
        car
    }

    pub fn set_variant(&mut self, variant: CarVariants) {
        match variant {
            CarVariants::Driving1 => {
                self.variant = CarVariants::Driving1;
                self.width = 32;
                self.height = 7;
            },
            CarVariants::Driving2 => {
                self.variant = CarVariants::Driving2;
                self.width = 32;
                self.height = 7;
            },
            CarVariants::Pushing1 => {
                self.variant = CarVariants::Pushing1;
                self.width = 40;
                self.height = 7;
            },
            CarVariants::Pushing2 => {
                self.variant = CarVariants::Pushing2;
                self.width = 40;
                self.height = 7;
            },
            CarVariants::Pulling1 => {
                self.variant = CarVariants::Pulling1;
                self.width = 68;
                self.height = 8;
            },
            CarVariants::Pulling2 => {
                self.variant = CarVariants::Pulling2;
                self.width = 68;
                self.height = 8;
            },
        }
    }

    pub fn switch_driving_variant(&mut self) {
        match self.variant {
            CarVariants::Driving1 => self.set_variant(CarVariants::Driving2),
            CarVariants::Driving2 => self.set_variant(CarVariants::Driving1),
            CarVariants::Pushing1 => self.set_variant(CarVariants::Pushing2),
            CarVariants::Pushing2 => self.set_variant(CarVariants::Pushing1),
            CarVariants::Pulling1 => self.set_variant(CarVariants::Pulling2),
            CarVariants::Pulling2 => self.set_variant(CarVariants::Pulling1),
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
        Self {
            height: 7,
            width: 32,
            variant: CarVariants::Driving1,
        }
    }
}

impl Widget for &Car {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        match self.variant {
            CarVariants::Driving1 => {
                return Paragraph::new(art::CAR_VARIANT1)
                    .left_aligned()
                    .render(area, buf);
            }
            CarVariants::Driving2 => {
                return Paragraph::new(art::CAR_VARIANT2)
                    .left_aligned()
                    .render(area, buf);
            }
            CarVariants::Pushing1 => {
                return Paragraph::new(art::CAR_PUSHING1)
                    .left_aligned()
                    .render(area, buf);
            }
            CarVariants::Pushing2 => {
                return Paragraph::new(art::CAR_PUSHING2)
                    .left_aligned()
                    .render(area, buf);
            }
            CarVariants::Pulling1 => {
                return Paragraph::new(art::CAR_PULLING1)
                    .left_aligned()
                    .render(area, buf);
            }
            CarVariants::Pulling2 => {
                return Paragraph::new(art::CAR_PULLING2)
                    .left_aligned()
                    .render(area, buf);
            }
        }
    }
}
