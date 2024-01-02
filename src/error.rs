use std::fmt;

use thiserror::Error;

pub enum Message {
    Max19DecimalPlaces,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Max19DecimalPlaces => "Warning: This function can only truncate to a maximum of 19 decimal places, truncating to maximum.",
        })
    }
}

pub enum Warning {
    F64(f64, Message),
    F32(f32, Message),
}

#[derive(Error, Debug)]
pub enum Error {}
