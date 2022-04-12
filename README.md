Peck lib.
A set of library and type functions.
```rust
pub mod str {
    pub const OS_PATH_SEPARATOR: &str = if cfg!(windows) { r"\" } else { "/" };
    pub const NUMS_CHARSET: &str = r"0123456789";
    pub const FLOAT_CHARSET: &str = r"0.-123456789";
    pub const ABS_FLOAT_CHARSET: &str = r".0123456789";
    pub fn trunc(input: &str, length: u8) -> &str {}
}

pub mod usize {
    pub const APPROX_CM_IN_ARC_SECOND: usize = 3087usize;
    pub const ARC_SECONDS_IN_360_DEGREES: usize = 1296000usize;
    pub const ARC_SECONDS_IN_360_DEGREES_INDEXED: usize = 1295999usize;
    pub const ARC_SECONDS_IN_180_DEGREES: usize = 648000usize;
    pub const ARC_SECONDS_IN_180_DEGREES_INDEXED: usize = 647999usize;
}

pub mod f64 {
    pub const LATITUDE_LIMIT: f64 = 90.0f64; //latitude (-90 to 90), lower = -LATITUDE_LIMIT, upper = LATITUDE_LIMIT
    pub const LONGITUDE_LIMIT: f64 = 180.0f64; //longitude (-180 to 180), lower = -LONGITUDE_LIMIT, upper = LONGITUDE_LIMIT
    pub const ARC_SECONDS_IN_360_DEGREES: f64 = 1296000.0f64;
    pub const ARC_SECONDS_IN_180_DEGREES: f64 = 648000.0f64;
    pub const EARTH_RADIUS_KM: f64 = 6378.137f64;
    pub const EARTH_RADIUS_M: f64 = 6378137.0f64;
    pub const DEG_TO_RAD: f64 = 0.017453292519943295f64;
    pub const RAD_TO_DEG: f64 = 57.29577951308232f64;
    pub fn rhs_exact(input: f64) -> f64 {}
    pub fn split(input: f64) -> (f64, f64) {}
    pub fn split_abs(input: f64) -> (f64, f64) {}
    pub fn to_radians(input_degrees: f64) -> f64 {}
    pub fn to_degrees(input_radians: f64) -> f64 {}
    pub fn normalise(input: f64, min: f64, max: f64) -> f64 {}
    pub fn normalised_to_index(input: f64, max: usize) -> usize {}
    pub fn indexify_lat(lat: f64) -> f64 {}
    pub fn indexify_long(long: f64) -> f64 {}
    pub fn indexify_lat_long(lat: f64, long: f64) -> (f64, f64) {}
    pub fn trunc(input: f64, decimal_places: u8) -> f64 {}
    pub fn trunc_safe(input: f64, decimal_places: u8) -> Result<f64, Warning> {}
    pub fn trunc_exact(input: f64, decimal_places: u8) -> f64 {}
    pub fn approx_equal_f64(a: f64, b: f64, decimal_places: u8) -> bool {}
    pub fn approx_equal_infallible_f64(a: f64, b: f64, decimal_places: u8) -> bool {}
}

pub mod f32 {
    pub const LATITUDE_LIMIT: f32 = 90.0f32; //latitude (-90 to 90), lower = -LATITUDE_LIMIT, upper = LATITUDE_LIMIT
    pub const LONGITUDE_LIMIT: f32 = 180.0f32; //longitude (-180 to 180), lower = -LONGITUDE_LIMIT, upper = LONGITUDE_LIMIT
    pub const ARC_SECONDS_IN_360_DEGREES: f32 = 1296000.0f32;
    pub const ARC_SECONDS_IN_180_DEGREES: f32 = 648000.0f32;
    pub const EARTH_RADIUS_KM: f32 = 6378.137f32;
    pub const EARTH_RADIUS_M: f32 = 6378137.0f32;
    pub const DEG_TO_RAD: f32 = 0.017453292f32;
    pub const RAD_TO_DEG: f32 = 57.29578f32; //57.295776
    pub fn rhs_exact(input: f32) -> f32 {}
    pub fn split(input: f32) -> (f32, f32) {}
    pub fn split_abs(input: f32) -> (f32, f32) {}
    pub fn to_radians(input_degrees: f32) -> f32 {}
    pub fn to_degrees(input_radians: f32) -> f32 {}
    pub fn normalise(input: f32, min: f32, max: f32) -> f32 {}
    pub fn normalised_to_index(input: f32, max: usize) -> usize {}
    pub fn indexify_lat(lat: f32) -> f32 {}
    pub fn indexify_long(long: f32) -> f32 {}
    pub fn indexify_lat_long(lat: f32, long: f32) -> (f32, f32) {}
    pub fn trunc(input: f32, decimal_places: u8) -> f32 {}
    pub fn trunc_safe(input: f32, decimal_places: u8) -> Result<f32, crate::error::Warning> {}
    pub fn trunc_exact(input: f32, decimal_places: u8) -> f32 {}
    pub fn approx_equal_f32(a: f32, b: f32, decimal_places: u8) -> bool {}
    pub fn approx_equal_infallible_f32(a: f32, b: f32, decimal_places: u8) -> bool {}
}

```