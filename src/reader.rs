//! This module is currently not good.
//!
//! It's beta stuff for eventual support for click events.
//! Do not use.
/* This module won't be used for now. Input is in a poor state */
use std::io::Read;
use std::char;

/// A reader that is used in the JSON parser
#[derive(Debug)]
pub struct Reader<R: Read>
{
    read_from: R,
}

impl<R: Read> Reader<R>
{
    /// Creates a new `Reader<R>` from an object that implements `Read`
    pub fn new(r: R) -> Reader<R>
    {
        Reader
        {
            read_from: r,
        }
    }
}

impl<R: Read> Iterator for Reader<R>
{
    type Item = char;

    fn next(&mut self) -> Option<char>
    {
        let mut buf = [0u8; 1];
        match self.read_from.read_exact(&mut buf)
        {
            Ok(_) =>
                Some(char::from_u32(buf[0] as u32).unwrap()),
            Err(_) =>
                None,
        }
    }
}
