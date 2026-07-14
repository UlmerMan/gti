use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{self, Constraint, Layout},
};

use crate::widgets::car::{Car, CarVariants};
use crate::widgets::force_hand::ForceHand;

pub struct App {
    car: Car,
    force: Option<ForceHand>,
    distance_driven: u16,
    exit: bool,
}

impl App {
    pub fn new(car_variant: CarVariants, force: bool) -> Self {
        Self {
            car: Car::new(car_variant),
            force: if force {
                Some(ForceHand::default())
            } else {
                None
            },
            distance_driven: 0,
            exit: false,
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
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        if !self.force.is_some() {
            let vertical = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(self.car.height),
                Constraint::Fill(1),
            ])
            .split(frame.area());

            if self.distance_driven == frame.area().width - self.car.width {
                self.exit();
            }

            let horizontal = Layout::default()
                .direction(layout::Direction::Horizontal)
                .constraints([
                    Constraint::Length(self.distance_driven),
                    Constraint::Length(self.car.width),
                    Constraint::Fill(1),
                ])
                .split(vertical[1]);

            if self.distance_driven % 4 == 0 {
                self.car.switch_driving_variant();
            }

            frame.render_widget(&self.car, horizontal[1]);
        } else {
            let vertical = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(self.force.as_ref().unwrap().height),
                Constraint::Fill(1),
            ])
            .split(frame.area());

            if self.distance_driven == frame.area().width - (self.car.width + self.force.as_ref().unwrap().width) {
                self.exit();
            }

            let horizontal = Layout::default()
                .direction(layout::Direction::Horizontal)
                .constraints([
                    Constraint::Length(self.force.as_ref().unwrap().width),
                    Constraint::Length(self.distance_driven),
                    Constraint::Length(self.car.width),
                    Constraint::Fill(1),
                ])
                .split(vertical[1]);
                
            frame.render_widget(&self.car, horizontal[2]);
            frame.render_widget(self.force.as_ref().unwrap(), horizontal[0]);
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
