pub mod error;
pub mod tests;

pub mod str {
    pub mod consts {
        pub const OS_PATH_SEPARATOR: &str = if cfg!(windows) { r"\" } else { "/" };
        pub const NUMS_CHARSET: &str = r"0123456789";
        pub const FLOAT_CHARSET: &str = r"0.-123456789";
        pub const ABS_FLOAT_CHARSET: &str = r".0123456789";
    }
    ///safely truncate a string to n length
    #[inline]
    pub fn trunc(input: &str, length: u8) -> &str {
        &input[..{
            if length as usize > input.len() {
                input.len()
            } else {
                length as usize
            }
        }]
    }
}

pub mod usize {
    pub mod consts {
        pub const APPROX_CM_IN_ARC_SECOND: usize = 3087usize;
        pub const ARC_SECONDS_IN_360_DEGREES: usize = 1296000usize;
        pub const ARC_SECONDS_IN_360_DEGREES_INDEXED: usize = 1295999usize;
        pub const ARC_SECONDS_IN_180_DEGREES: usize = 648000usize;
        pub const ARC_SECONDS_IN_180_DEGREES_INDEXED: usize = 647999usize;
    }
}

pub mod f64 {
    use crate::error::{Message, Warning};

    use self::consts::{DEG_TO_RAD, RAD_TO_DEG};

    pub mod consts {
        pub const LATITUDE_LIMIT: f64 = 90.0f64; //latitude (-90 to 90), lower = -LATITUDE_LIMIT, upper = LATITUDE_LIMIT
        pub const LONGITUDE_LIMIT: f64 = 180.0f64; //longitude (-180 to 180), lower = -LONGITUDE_LIMIT, upper = LONGITUDE_LIMIT
        pub const ARC_SECONDS_IN_360_DEGREES: f64 = 1296000.0f64;
        pub const ARC_SECONDS_IN_180_DEGREES: f64 = 648000.0f64;
        pub const EARTH_RADIUS_KM: f64 = 6378.137f64;
        pub const EARTH_RADIUS_M: f64 = 6378137.0f64;
        pub const DEG_TO_RAD: f64 = 0.017453292519943295f64;
        pub const RAD_TO_DEG: f64 = 57.29577951308232f64;
    }

    ///right hand side of the decimal point exactly as it would display, it won't gain any apparent precision when removing the exponent which gives the type more precision
    #[inline]
    pub fn rhs_exact(input: f64) -> f64 {
        let input_string: String = input.to_string();
        if let Some((_, rhs_str)) = input_string.split_once('.') {
            format!("0.{}", rhs_str).parse::<f64>().unwrap()
        } else {
            0.0f64
        }
    }

    ///split left and right side of the decimal point
    #[inline]
    pub fn split(input: f64) -> (f64, f64) {
        (input.trunc(), input.fract())
    }

    ///split left and right side of the decimal point absolute
    #[inline]
    pub fn split_abs(input: f64) -> (f64, f64) {
        (input.abs().trunc(), input.abs().fract())
    }

    ///converting an implied degree value to radians
    #[inline]
    pub fn to_radians(input_degrees: f64) -> f64 {
        input_degrees * DEG_TO_RAD
    }

    ///converting an implied radians value to degrees
    #[inline]
    pub fn to_degrees(input_radians: f64) -> f64 {
        input_radians * RAD_TO_DEG
    }

    ///normalise a value between the minimum and maximum value it could be
    #[inline]
    pub fn normalise(input: f64, min: f64, max: f64) -> f64 {
        (input - min) / (max - min)
    }

    ///get the index using a value normalised to the dimensions of your vector, array, or other storage indexed or sized structures
    #[inline]
    pub fn normalised_to_index(input: f64, max: usize) -> usize {
        (max as f64 * input) as usize
    }

    ///shifts the value from (-90 to 90) to (0 to 180)
    #[inline]
    pub fn indexify_lat(lat: f64) -> f64 {
        lat + 90.0f64
    }

    ///shifts the value from (-180 to 180) to (0 to 360)
    #[inline]
    pub fn indexify_long(long: f64) -> f64 {
        long + 180.0f64
    }

    ///shifts lat value from (-90 to 90) to (0 to 180) and long value from (-180 to 180) to (0 to 360)
    #[inline]
    pub fn indexify_lat_long(lat: f64, long: f64) -> (f64, f64) {
        (indexify_lat(lat), indexify_long(long))
    }

    ///can only truncate to 19 decimal places safely
    ///a normalised (between 0-1) f64 value will have a maximum of 8 significant digits after the decimal place
    #[inline]
    pub fn trunc(input: f64, decimal_places: u8) -> f64 {
        let factor: f64 = 10usize.pow(decimal_places as u32) as f64;
        let output_abs: f64 = (input.abs() * factor).floor() / factor;
        output_abs.copysign(input)
    }

    #[inline]
    #[allow(clippy::nonminimal_bool)]
    pub fn trunc_safe(input: f64, decimal_places: u8) -> Result<f64, Warning> {
        let mut safe: bool = true;
        let factor: f64 = 10usize.pow({
            if !(decimal_places > 19u8) {
                decimal_places as u32
            } else {
                safe = false;
                19u32
            }
        }) as f64;
        let output_abs: f64 = (input.abs() * factor).floor() / factor;
        let output: f64 = output_abs.copysign(input);
        match safe {
            true => Ok(output),
            false => {
                //not actually an error, but Result requires Ok() or Err()
                //it passes through an enumerated message which implements display
                //it currently only warns that it could only truncate to 19 decimal places
                //and returns it as such.
                Err(Warning::F64(output, Message::Max19DecimalPlaces))
            }
        }
    }

    ///no rounding
    ///a normalised (between 0-1) f64 value will have a maximum of 16 significant digits after the decimal place
    #[inline]
    pub fn trunc_exact(input: f64, decimal_places: u8) -> f64 {
        let input_string: String = input.to_string();
        if let Some((lhs_str, rhs_str)) = input_string.split_once('.') {
            let rhs_string: String = rhs_str
                .chars()
                .into_iter()
                .take(decimal_places as usize)
                .collect();
            format!("{}.{}", lhs_str, rhs_string)
                .parse::<f64>()
                .unwrap()
        } else {
            input.trunc()
        }
    }

    #[inline]
    pub fn approx_equal_f64(a: f64, b: f64, decimal_places: u8) -> bool {
        let factor: f64 = 10usize.pow(decimal_places as u32) as f64;
        (a * factor).trunc() == (b * factor).trunc()
    }

    #[inline]
    #[allow(clippy::unnecessary_unwrap)]
    ///infallible, but significantly slower, 633ns vs 37ns
    pub fn approx_equal_infallible_f64(a: f64, b: f64, decimal_places: u8) -> bool {
        //lhs short circuit
        if a as isize != b as isize {
            return false;
        }

        let a_string: String = a.to_string();
        let b_string: String = b.to_string();

        let a_string_split: Option<(&str, &str)> = a_string.split_once('.');
        let b_string_split: Option<(&str, &str)> = b_string.split_once('.');
        if a_string_split.is_some() && b_string_split.is_some() {
            let (_, a_rhs) = a_string_split.unwrap();
            let (_, b_rhs) = b_string_split.unwrap();
            return crate::str::trunc(a_rhs, decimal_places)
                == crate::str::trunc(b_rhs, decimal_places);
        } else if a_string_split.is_none() && b_string_split.is_none() {
            return true;
        }

        false
    }
}

pub mod f32 {
    use self::consts::{DEG_TO_RAD, RAD_TO_DEG};

    pub mod consts {
        pub const LATITUDE_LIMIT: f32 = 90.0f32; //latitude (-90 to 90), lower = -LATITUDE_LIMIT, upper = LATITUDE_LIMIT
        pub const LONGITUDE_LIMIT: f32 = 180.0f32; //longitude (-180 to 180), lower = -LONGITUDE_LIMIT, upper = LONGITUDE_LIMIT
        pub const ARC_SECONDS_IN_360_DEGREES: f32 = 1296000.0f32;
        pub const ARC_SECONDS_IN_180_DEGREES: f32 = 648000.0f32;
        pub const EARTH_RADIUS_KM: f32 = 6378.137f32;
        pub const EARTH_RADIUS_M: f32 = 6378137.0f32;
        pub const DEG_TO_RAD: f32 = 0.017453292f32;
        pub const RAD_TO_DEG: f32 = 57.29578f32; //57.295776
    }

    ///right hand side of the decimal point exactly as it would display, it won't gain any apparent precision when removing the exponent which gives the type more precision
    #[inline]
    pub fn rhs_exact(input: f32) -> f32 {
        let input_string: String = input.to_string();
        if let Some((_, rhs_str)) = input_string.split_once('.') {
            format!("0.{}", rhs_str).parse::<f32>().unwrap()
        } else {
            0.0f32
        }
    }

    ///split left and right side of the decimal point
    #[inline]
    pub fn split(input: f32) -> (f32, f32) {
        (input.trunc(), input.fract())
    }

    ///split left and right side of the decimal point absolute
    #[inline]
    pub fn split_abs(input: f32) -> (f32, f32) {
        (input.abs().trunc(), input.abs().fract())
    }

    ///converting an implied degree value to radians
    #[inline]
    pub fn to_radians(input_degrees: f32) -> f32 {
        input_degrees * DEG_TO_RAD
    }

    ///converting an implied radians value to degrees
    #[inline]
    pub fn to_degrees(input_radians: f32) -> f32 {
        input_radians * RAD_TO_DEG
    }

    ///normalise a value between the minimum and maximum value it could be
    #[inline]
    pub fn normalise(input: f32, min: f32, max: f32) -> f32 {
        (input - min) / (max - min)
    }

    ///get the index using a value normalised to the dimensions of your vector, array, or other storage indexed or sized structures
    #[inline]
    pub fn normalised_to_index(input: f32, max: usize) -> usize {
        (max as f32 * input) as usize
    }

    ///shifts the value from (-90 to 90) to (0 to 180)
    #[inline]
    pub fn indexify_lat(lat: f32) -> f32 {
        lat + 90.0
    }

    ///shifts the value from (-180 to 180) to (0 to 360)
    #[inline]
    pub fn indexify_long(long: f32) -> f32 {
        long + 180.0
    }

    ///shifts lat value from (-90 to 90) to (0 to 180) and long value from (-180 to 180) to (0 to 360)
    #[inline]
    pub fn indexify_lat_long(lat: f32, long: f32) -> (f32, f32) {
        (indexify_lat(lat), indexify_long(long))
    }

    ///can only truncate to 8 decimal places safely
    ///a normalised (between 0-1) f64 value will have a maximum of 8 significant digits after the decimal place
    #[inline]
    pub fn trunc(input: f32, decimal_places: u8) -> f32 {
        let factor: f32 = 10usize.pow(decimal_places as u32) as f32;
        let output_abs: f32 = (input.abs() * factor).floor() / factor;
        output_abs.copysign(input)
    }

    #[inline]
    #[allow(clippy::nonminimal_bool)]
    pub fn trunc_safe(input: f32, decimal_places: u8) -> Result<f32, crate::error::Warning> {
        let mut safe: bool = true;
        let factor: f32 = 10usize.pow({
            if !(decimal_places > 19u8) {
                decimal_places as u32
            } else {
                safe = false;
                19u32
            }
        }) as f32;
        let output_abs: f32 = (input.abs() * factor).floor() / factor;
        let output: f32 = output_abs.copysign(input);
        match safe {
            true => Ok(output),
            false => {
                //not actually an error, but Result requires Ok() or Err()
                //it passes through an enumerated message which implements display
                //it currently only warns that it could only truncate to 19 decimal places
                //and returns it as such.
                Err(crate::error::Warning::F32(
                    output,
                    crate::error::Message::Max19DecimalPlaces,
                ))
            }
        }
    }

    ///a normalised (between 0-1) f64 value will have a maximum of 16 values after the decimal place
    ///no rounding
    #[inline]
    pub fn trunc_exact(input: f32, decimal_places: u8) -> f32 {
        let input_string: String = input.to_string();
        if let Some((lhs_str, rhs_str)) = input_string.split_once('.') {
            let rhs_string: String = rhs_str
                .chars()
                .into_iter()
                .take(decimal_places as usize)
                .collect();
            format!("{}.{}", lhs_str, rhs_string)
                .parse::<f32>()
                .unwrap()
        } else {
            input.trunc()
        }
    }

    #[inline]
    pub fn approx_equal_f32(a: f32, b: f32, decimal_places: u8) -> bool {
        let factor: f32 = 10usize.pow(decimal_places as u32) as f32;
        (a * factor).trunc() == (b * factor).trunc()
    }

    #[inline]
    #[allow(clippy::unnecessary_unwrap)]
    pub fn approx_equal_infallible_f32(a: f32, b: f32, decimal_places: u8) -> bool {
        //lhs short circuit
        if a as isize != b as isize {
            return false;
        }

        let a_string: String = a.to_string();
        let b_string: String = b.to_string();

        let a_string_split: Option<(&str, &str)> = a_string.split_once('.');
        let b_string_split: Option<(&str, &str)> = b_string.split_once('.');
        if a_string_split.is_some() && b_string_split.is_some() {
            let (_, a_rhs) = a_string_split.unwrap();
            let (_, b_rhs) = b_string_split.unwrap();
            return crate::str::trunc(a_rhs, decimal_places)
                == crate::str::trunc(b_rhs, decimal_places);
        } else if a_string_split.is_none() && b_string_split.is_none() {
            return true;
        }

        false
    }
}
