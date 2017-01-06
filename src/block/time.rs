//! A quick time block. Spits out the output of strftime and updates once a
//! second.

use block::{Block, Status};
use chrono::offset::local::Local;
use std::time::Duration;

#[derive(Debug)]
pub struct Time<'a>
{
    time: String,
    format: &'a str,
}

impl<'a> Time<'a>
{
    pub fn new(format: &'a str) -> Time<'a>
    {
        Time
        {
            time: String::from(""),
            format: format,
        }
    }
}


impl<'a> Block for Time<'a>
{
    fn update(&mut self) -> Duration
    {
        self.time = format!("{}", Local::now().format(self.format));
        Duration::new(1, 0)
    }

    fn get_status(&self) -> Status
    {
        Status::new(self.time.clone())
    }

    fn click_callback(&mut self)
    {
    }
}
