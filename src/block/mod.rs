pub mod time;
pub use self::time::Time;

pub mod net_usage;
pub use self::net_usage::NetUsage;

/* TODO: Remove this use */
use std::fmt;
use std::time::Duration;
use rustc_serialize::{Encoder, Encodable};

/*********/
/* Color */
/*********/
/// A struct for specifying color for various parts of the status.
#[derive(Debug)]
pub struct Color(pub u8, pub u8, pub u8);

impl Encodable for Color
{
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>
    {
        s.emit_str(&format!("#{:X}{:X}{:X}", self.0, self.1, self.2))
    }
}

/***************/
/* StatusAlign */
/***************/
/// An enum for specifying the alignment of a status.
#[derive(Debug,RustcEncodable)]
pub enum StatusAlign
{
    Center,
    Left,
    Right,
}

/****************/
/* StatusMarkup */
/****************/
/// An enum for specifying the markup of a status.
#[derive(Debug,RustcEncodable)]
pub enum StatusMarkup
{
    Pango,
    None,
}

/**********/
/* Status */
/**********/
/// A struct for specifying the status of a block.
///
/// This maps directly to the API that i3bar uses to display data.
///
/// The API can be found [here](https://i3wm.org/docs/i3bar-protocol.html)
#[derive(Debug,RustcEncodable)]
pub struct Status
{
    pub full_text: String,
    pub short_text: Option<String>,
    pub color: Option<Color>,
    pub background: Option<Color>,
    pub border: Option<Color>,
    pub min_width: Option<u64>,
    pub align: Option<StatusAlign>,
    pub name: Option<String>,
    pub instance: Option<String>,
    pub urgent: Option<bool>,
    pub separator: Option<bool>,
    pub separator_block_width: Option<u64>,
    pub markup: Option<StatusMarkup>,
}

impl Status
{
    /// Creates a new `Status` with default values.
    pub fn new(full_text: String) -> Status
    {
        Status
        {
            full_text: full_text,
            short_text: None,
            color: None,
            background: None,
            border: None,
            min_width: None,
            align: None,
            name: None,
            instance: None,
            urgent: None,
            separator: None,
            separator_block_width: None,
            markup: None,
        }
    }
}

/*********/
/* Block */
/*********/
/// Trait for status blocks.
///
/// Each block in the resulting status is tied to a `Block`. Each block gets
/// updated via its `update` method and its status is returned via the 
/// `get_status` method.
///
/// The `click_callback` function is currently not used, but it eventually
/// will be called when i3bar sends a click event to the block.
pub trait Block: fmt::Debug
{
    /// Updates the `Block`. Returns a `Duration` that represents how long
    /// to wait till the next update call.
    fn update(&mut self) -> Duration;

    /// Returns the status of the block.
    ///
    /// **Note**: Little to no processing should be done in this method.
    /// Processing should all be done in the `update` method.
    fn get_status(&self) -> Status;

    /// **UNIMPLEMENTED**
    fn click_callback(&mut self);
}
