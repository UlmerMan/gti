use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
};

use crate::widgets::force_hand::ForceHand;
use crate::widgets::{
    binoculars,
    car::{Car, CarVariants},
};

pub struct App {
    using_force: bool,
    using_binoculars: bool,
    car_variant: CarVariants,
    distance_driven: u16,
    exit: bool,
}

impl App {
    pub fn new(using_force: bool, using_binoculars: bool, car_variant: CarVariants, distance_driven: u16, exit: bool) -> Self {
        Self {
            using_force: using_force,
            using_binoculars: using_binoculars,
            car_variant: car_variant,
            distance_driven: distance_driven,
            exit: exit,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        let mut last_tick = Instant::now();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            self.handle_events()?;

            // Tick once per second.
            if last_tick.elapsed() >= Duration::from_millis(50) {
                self.distance_driven += 1;
                self.update();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut car = Car::default();

        let hand = if self.using_force {
            Some(ForceHand::default())
        } else {
            None
        };

        let binoculars = binoculars::Binoculars::new();

        car.set_variant(self.car_variant);

        let horizontal_width = car.width
            + if self.using_force {
                hand.as_ref().map_or(0, |h| h.width)
            } else {
                0
            };

        if self.distance_driven >= frame.area().width.saturating_sub(horizontal_width) {
            self.exit();
        }

        let vertical = if self.using_force {
            let hand = hand.as_ref().unwrap();

            Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(hand.height),
                Constraint::Fill(1),
            ])
        } else {
            Layout::vertical([
                if self.using_binoculars && binoculars.height + car.height <= frame.area().height {
                    Constraint::Length(binoculars.height)
                } else {
                    Constraint::Fill(1)
                },
                Constraint::Length(car.height),
                Constraint::Fill(1),
            ])
        };

        let horizontal = if self.using_force {
            let hand = hand.as_ref().unwrap();

            Layout::horizontal([
                Constraint::Length(hand.width),
                Constraint::Length(self.distance_driven),
                Constraint::Length(car.width),
                Constraint::Fill(1),
            ])
        } else {
            Layout::horizontal([
                Constraint::Length(self.distance_driven),
                Constraint::Length(car.width),
                Constraint::Fill(1),
            ])
        };

        let vertical_split = vertical.split(frame.area());
        let horizontal_split = horizontal.split(vertical_split[1]);

        if self.using_force {
            frame.render_widget(hand.as_ref().unwrap(), horizontal_split[0]);
            frame.render_widget(&car, horizontal_split[2]);
        } else {
            frame.render_widget(&car, horizontal_split[1]);
        }

        if self.using_binoculars && binoculars.height + car.height <= frame.area().height {
            frame.render_widget(&binoculars, vertical_split[0]);
        }
    }

    fn update(&mut self) {
        if self.distance_driven % 4 == 0 {
            self.car_variant.switch_driving_variant();
        }
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key_event(key);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            using_force: false,
            using_binoculars: false,
            car_variant: CarVariants::Driving,
            distance_driven: 0,
            exit: false,
        }
    }
}

impl From<clap::ArgMatches> for App {
    fn from(parameters: clap::ArgMatches) -> Self {
        let mut app = App::default();

        match parameters.subcommand() {
            Some(("push", sub_matches)) => {
                if sub_matches.get_flag("force") {
                    app.car_variant = CarVariants::Driving;
                    app.using_force = true;
                } else {
                    app.car_variant = CarVariants::Pushing1;
                }
            }
            Some(("pull", _)) => {
                app.car_variant = CarVariants::Pulling1;
            }
            Some(("checkout", _)) => {
                app.car_variant = CarVariants::Driving1;
                app.using_binoculars = true;
            }
            _ => {
                app.car_variant = CarVariants::Driving1;
            }
        }

        app
    }
}