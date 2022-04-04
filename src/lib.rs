pub mod testing;
pub mod consts {
    pub const OS_PATH_SEPARATOR: &str = if cfg!(windows) { r"\" } else { "/" };
    pub const NUMS: &str = r"0123456789";
    pub const FLOAT_CHARSET: &str = r"0.-123456789";
    pub const ABS_FLOAT_CHARSET: &str = r"0.123456789";
}

pub mod usize {
    pub mod consts {
        pub const APPROX_CM_IN_SECOND: usize = 3087;
        pub const ARC_SECONDS_IN_360_DEGREES_USIZE: usize = 1296000;
        pub const ARC_SECONDS_IN_360_DEGREES_INDEXED: usize = 1295999;
        pub const ARC_SECONDS_IN_180_DEGREES_USIZE: usize = 648000;
        pub const ARC_SECONDS_IN_180_DEGREES_INDEXED: usize = 647999;
    }
}

pub mod f64 {
    use self::consts::{DEG_TO_RAD, RAD_TO_DEG};

    pub mod consts {
        use std::f64::consts::PI;

        pub const LATITUDE_LIMIT: f64 = 90.0; //latitude (-90 to 90), lower = -LATITUDE_LIMIT, upper = LATITUDE_LIMIT
        pub const LONGITUDE_LIMIT: f64 = 180.0; //longitude (-180 to 180), lower = -LONGITUDE_LIMIT, upper = LONGITUDE_LIMIT
        pub const ARC_SECONDS_IN_360_DEGREES_F64: f64 = 1296000.0;
        pub const ARC_SECONDS_IN_180_DEGREES_F64: f64 = 648000.0;
        pub const EARTH_RADIUS_KM: f64 = 6378.137;
        pub const EARTH_RADIUS_M: f64 = 6378137.0;
        pub const DEG_TO_RAD: f64 = PI / 180.0;
        pub const RAD_TO_DEG: f64 = 57.2957795130823209;
        pub const TWO_PI: f64 = PI + PI;
    }

    ///right hand side of the decimal point
    #[inline]
    pub fn rhs(input: f64) -> f64 {
        let input_abs = input.abs();
        input_abs - input_abs as usize as f64
    }
    
    ///left hand side of the decimal point
    #[inline]
    pub fn lhs(input: f64) -> f64 {
        input as isize as f64
    }

    ///left hand side of the decimal point absolute 
    #[inline]
    pub fn lhs_abs(input: f64) -> f64 {
        input.abs() as usize as f64
    }

    ///split left and right side of the decimal point
    #[inline]
    pub fn split(input: f64) -> (f64, f64) {
        (lhs(input), rhs(input))
    }

    ///split left and right side of the decimal point absolute
    #[inline]
    pub fn split_abs(input: f64) -> (f64, f64) {
        (lhs(input.abs()), rhs(input))
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
        lat.abs() + 90.0
    }
    
    ///shifts the value from (-180 to 180) to (0 to 360)
    #[inline]
    pub fn indexify_long(long: f64) -> f64 {
        long.abs() + 180.0
    }
    
    ///shifts lat value from (-90 to 90) to (0 to 180) and long value from (-180 to 180) to (0 to 360)
    #[inline]
    pub fn indexify_lat_long(lat: f64, long: f64) -> (f64, f64) {
        (indexify_lat(lat), indexify_long(long))
    }
}

pub mod f32 {
    use self::consts::{RAD_TO_DEG, DEG_TO_RAD};

    pub mod consts {
        use std::f32::consts::PI;

        pub const LATITUDE_LIMIT: f32 = 90.0; //latitude (-90 to 90), lower = -LATITUDE_LIMIT, upper = LATITUDE_LIMIT
        pub const LONGITUDE_LIMIT: f32 = 180.0; //longitude (-180 to 180), lower = -LONGITUDE_LIMIT, upper = LONGITUDE_LIMIT
        pub const ARC_SECONDS_IN_360_DEGREES_F32: f32 = 1296000.0;
        pub const ARC_SECONDS_IN_180_DEGREES_F32: f32 = 648000.0;
        pub const EARTH_RADIUS_KM: f32 = 6378.137;
        pub const EARTH_RADIUS_M: f32 = 6378137.0;
        pub const DEG_TO_RAD: f32 = PI / 180.0;
        pub const RAD_TO_DEG: f32 = 57.2957795130823209;
        pub const TWO_PI: f32 = PI + PI;
    }
    ///right hand side of the decimal point
    #[inline]
    pub fn rhs(input: f32) -> f32 {
        let input_abs = input.abs();
        input_abs - input_abs as usize as f32
    }
    
    ///left hand side of the decimal point
    #[inline]
    pub fn lhs(input: f32) -> f32 {
        input as isize as f32
    }

    ///left hand side of the decimal point absolute
    #[inline]
    pub fn lhs_abs(input: f32) -> f32 {
        input.abs() as usize as f32
    }

    ///split left and right side of the decimal point
    #[inline]
    pub fn split(input: f32) -> (f32, f32) {
        (lhs(input), rhs(input))
    }

    ///split left and right side of the decimal point absolute
    #[inline]
    pub fn split_abs(input: f32) -> (f32, f32) {
        (lhs(input.abs()), rhs(input))
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
        lat.abs() + 90.0
    }
    
    ///shifts the value from (-180 to 180) to (0 to 360)
    #[inline]
    pub fn indexify_long(long: f32) -> f32 {
        long.abs() + 180.0
    }
    
    ///shifts lat value from (-90 to 90) to (0 to 180) and long value from (-180 to 180) to (0 to 360)
    #[inline]
    pub fn indexify_lat_long(lat: f32, long: f32) -> (f32, f32) {
        (indexify_lat(lat), indexify_long(long))
    }
}