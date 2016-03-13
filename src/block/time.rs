//! A quick time block. Spits out the output of ctime and updates once a
//! second.

use block::{Block, Status};
use time;
use std::time::Duration;

#[derive(Debug)]
pub struct Time
{
    time: String,
}

impl Time
{
    pub fn new() -> Time
    {
        Time
        {
            time: String::from(""),
        }
    }
}


impl Block for Time
{
    fn update(&mut self) -> Duration
    {
        let now = time::now();
        self.time = format!("{}", now.ctime());
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
