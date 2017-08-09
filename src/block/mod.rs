pub mod time;
pub use self::time::Time;

pub mod net_usage;
pub use self::net_usage::NetUsage;

/* TODO: Remove this use */
use std::fmt;
use std::time::Duration;

use serde::{Serialize, Serializer};

/*********/
/* Color */
/*********/
/// A struct for specifying color for various parts of the status.
#[derive(Clone,Copy,Debug)]
pub struct Color(pub u8, pub u8, pub u8);

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2))
    }
}

/***************/
/* StatusAlign */
/***************/
/// An enum for specifying the alignment of a status.
#[derive(Clone,Copy,Debug,Serialize)]
pub enum StatusAlign {
    Center,
    Left,
    Right,
}

/****************/
/* StatusMarkup */
/****************/
/// An enum for specifying the markup of a status.
#[derive(Clone,Copy,Debug,Serialize)]
pub enum StatusMarkup {
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
#[derive(Clone,Debug,Serialize)]
pub struct Status {
    pub full_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<StatusAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_block_width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<StatusMarkup>,
}

impl Status {
    /// Creates a new `Status` with default values.
    pub fn new(full_text: String) -> Status {
        Status {
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
pub trait Block: fmt::Debug {
    /// Updates the `Block`. Returns a `Duration` that represents how long
    /// to wait till the next update call.
    fn update(&mut self) -> (Status, Duration);

    /// **UNIMPLEMENTED**
    fn click_callback(&mut self);
}
