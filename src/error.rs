use std::fmt;

pub enum Message {
    Max9DecimalPlaces,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Max9DecimalPlaces => format!("Warning: This function can only truncate to a maximum of 9 decimal places, truncating to maximum."),
        })
    }
}

pub enum Error {

}

pub enum Warning {
    F64(f64, Message),
    F32(f32, Message),
}