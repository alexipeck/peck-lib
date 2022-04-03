pub fn indexify_lat(lat: f64) -> f64 {
    lat.abs() + 90.0
}

pub fn indexify_long(long: f64) -> f64 {
    long.abs() + 180.0
}

pub fn indexify_lat_long(lat: f64, long: f64) -> (f64, f64) {
    (indexify_lat(lat), indexify_long(long))
}