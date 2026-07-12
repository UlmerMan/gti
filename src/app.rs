use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{self, Constraint, Layout},
};

use crate::widgets::car::Car;

pub struct App {
    car: Car,
    distance_driven: u16,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            car: Car::default(),
            distance_driven: 0,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
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
        if self.distance_driven == frame.area().width - self.car.width {
            self.exit();
        }

        let vertical = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(self.car.height),
                Constraint::Fill(1),
            ])
            .split(frame.area());

        let horizontal = Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints([
                Constraint::Length(self.distance_driven),
                Constraint::Length(self.car.width),
                Constraint::Fill(1),
            ])
            .split(vertical[1]);
        
        if self.distance_driven % 4 == 0 {
            self.car.switch_variant();
        } 

        frame.render_widget(&self.car, horizontal[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
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
