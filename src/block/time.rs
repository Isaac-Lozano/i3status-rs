//! A quick time block. Spits out the output of strftime and updates once a
//! second.

use block::{Block, Status};
use chrono::offset::local::Local;
use std::time::Duration;

#[derive(Debug)]
pub struct Time<'a> {
    format: &'a str,
}

impl<'a> Time<'a> {
    pub fn new(format: &'a str) -> Time<'a> {
        Time { format: format }
    }
}


impl<'a> Block for Time<'a> {
    fn update(&mut self) -> (Status, Duration) {
        let time = format!("{}", Local::now().format(self.format));
        (Status::new(time), Duration::new(1, 0))
    }

    fn click_callback(&mut self) {}
}
