//#[cfg(test)]
#[allow(unused_imports)]
use crate::error::Warning;
#[allow(unused_imports)]
use crate::f64::consts::RAD_TO_DEG as RAD_TO_DEG_F64;
#[allow(unused_imports)]
use crate::f32::consts::RAD_TO_DEG as RAD_TO_DEG_F32;
#[allow(unused_imports)]
use crate::f64::trunc_exact;

//based on: https://stackoverflow.com/questions/41447678/comparison-of-two-floats-in-rust-to-arbitrary-level-of-precision
#[allow(dead_code)]
fn approx_equal_f64(a: f64, b: f64, decimal_places: u8) -> bool {
    let factor = 10usize.pow(decimal_places as u32) as f64;
    (a * factor).trunc() == (b * factor).trunc()
}

#[allow(dead_code)]
fn approx_equal_f32(a: f32, b: f32, decimal_places: u8) -> bool {
    let factor = 10usize.pow(decimal_places as u32) as f32;
    (a * factor).trunc() == (b * factor).trunc()
}

#[test]
fn test_rhs_f64() {
    assert!(approx_equal_f64(
        crate::f64::rhs(RAD_TO_DEG_F64),
        0.29577951308232,
        13
    ));
    assert!(approx_equal_f64(
        crate::f64::rhs(-RAD_TO_DEG_F64),
        0.29577951308232,
        13
    ));
}

#[test]
fn test_rhs_f32() {
    assert!(approx_equal_f32(crate::f32::rhs(RAD_TO_DEG_F32), 0.29578, 5));
    assert!(approx_equal_f32(crate::f32::rhs(-RAD_TO_DEG_F32), 0.29578, 5));
}

#[test]
fn test_lhs_f64() {
    assert_eq!(crate::f64::lhs(RAD_TO_DEG_F64), 57.0);
    assert_eq!(crate::f64::lhs(-RAD_TO_DEG_F64), -57.0);
}

#[test]
fn test_lhs_f32() {
    assert_eq!(crate::f32::lhs(RAD_TO_DEG_F32), 57.0);
    assert_eq!(crate::f32::lhs(-RAD_TO_DEG_F32), -57.0);
}

#[test]
fn test_lhs_abs_f64() {
    assert_eq!(crate::f64::lhs_abs(RAD_TO_DEG_F64), 57.0);
    assert_eq!(crate::f64::lhs_abs(-RAD_TO_DEG_F64), 57.0);
}

#[test]
fn test_lhs_abs_f32() {
    assert_eq!(crate::f32::lhs_abs(RAD_TO_DEG_F32), 57.0);
    assert_eq!(crate::f32::lhs_abs(-RAD_TO_DEG_F32), 57.0);
}

#[test]
fn test_split_f64() {
    {
        let (lhs, rhs) = crate::f64::split(RAD_TO_DEG_F64);
        assert_eq!(lhs, 57.0);
        assert!(approx_equal_f64(rhs, 0.29577951308232, 13));
    }
    {
        let (lhs, rhs) = crate::f64::split(-RAD_TO_DEG_F64);
        assert_eq!(lhs, -57.0);
        assert!(approx_equal_f64(rhs, 0.29577951308232, 13));
    }
}

#[test]
fn test_split_f32() {
    {
        let (lhs, rhs) = crate::f32::split(RAD_TO_DEG_F32);
        assert_eq!(lhs, 57.0);
        assert!(approx_equal_f32(rhs, 0.29578, 5));
    }
    {
        let (lhs, rhs) = crate::f32::split(-RAD_TO_DEG_F32);
        assert_eq!(lhs, -57.0);
        assert!(approx_equal_f32(rhs, 0.29578, 5));
    }
}

#[test]
fn test_split_abs_f64() {
    {
        let (lhs, rhs) = crate::f64::split_abs(RAD_TO_DEG_F64);
        assert_eq!(lhs, 57.0);
        assert!(approx_equal_f64(rhs, 0.29577951308232, 13));
    }
    {
        let (lhs, rhs) = crate::f64::split_abs(-RAD_TO_DEG_F64);
        assert_eq!(lhs, 57.0);
        assert!(approx_equal_f64(rhs, 0.29577951308232, 13));
    }
}

#[test]
fn test_split_abs_f32() {
    {
        let (lhs, rhs) = crate::f32::split_abs(RAD_TO_DEG_F32);
        assert_eq!(lhs, 57.0);
        assert!(approx_equal_f32(rhs, 0.29578, 5));
    }
    {
        let (lhs, rhs) = crate::f32::split_abs(-RAD_TO_DEG_F32);
        assert_eq!(lhs, 57.0);
        assert!(approx_equal_f32(rhs, 0.29578, 5));
    }
}

#[test]
fn test_to_radians_f64() {
    let input: f64 = 100.0; //100.0000000003249
    assert_eq!(input.to_radians(), crate::f64::to_radians(input));
    assert_eq!(crate::f64::to_radians(input), 1.7453292519943295);
    assert_eq!(crate::f64::to_radians(-input), -1.7453292519943295);
}

#[test]
fn test_to_radians_f32() {
    let input: f32 = 100.0; //100.0000000003249
    assert_eq!(input.to_radians(), crate::f32::to_radians(input));
    assert_eq!(crate::f32::to_radians(input), 1.7453293);
    assert_eq!(crate::f32::to_radians(-input), -1.7453293);
}

#[test]
fn test_to_degrees_f64() {
    let num: f64 = 1.745329252;
    assert_eq!(num.to_degrees(), crate::f64::to_degrees(num));
    assert_eq!(crate::f64::to_degrees(num), 100.0000000003249);
    assert_eq!(crate::f64::to_degrees(-num), -100.0000000003249);
}

#[test]
fn test_to_degrees_f32() {
    let num: f32 = 1.7453293;
    assert_eq!(num.to_degrees(), crate::f32::to_degrees(num));
    assert_eq!(crate::f32::to_degrees(num), 100.0000000003249);
    assert_eq!(crate::f32::to_degrees(-num), -100.0000000003249);
}

#[test]
fn test_normalise_f64() {
    assert_eq!(
        crate::f64::normalise(
            crate::f64::consts::ARC_SECONDS_IN_360_DEGREES / 2.0,
            0.0,
            crate::f64::consts::ARC_SECONDS_IN_360_DEGREES
        ),
        0.5
    );
    assert_eq!(
        crate::f64::normalise(
            crate::f64::consts::ARC_SECONDS_IN_360_DEGREES,
            0.0,
            crate::f64::consts::ARC_SECONDS_IN_360_DEGREES
        ),
        1.0
    );
    assert_eq!(
        crate::f64::normalise(0.0, 0.0, crate::f64::consts::ARC_SECONDS_IN_360_DEGREES),
        0.0
    );
}

#[test]
fn test_normalise_f32() {
    assert_eq!(
        crate::f32::normalise(
            crate::f32::consts::ARC_SECONDS_IN_360_DEGREES / 2.0,
            0.0,
            crate::f32::consts::ARC_SECONDS_IN_360_DEGREES
        ),
        0.5
    );
    assert_eq!(
        crate::f32::normalise(
            crate::f32::consts::ARC_SECONDS_IN_360_DEGREES,
            0.0,
            crate::f32::consts::ARC_SECONDS_IN_360_DEGREES
        ),
        1.0
    );
    assert_eq!(
        crate::f32::normalise(0.0, 0.0, crate::f32::consts::ARC_SECONDS_IN_360_DEGREES),
        0.0
    );
}

#[test]
fn test_normalised_to_index_f64() {
    assert_eq!(
        crate::f64::normalised_to_index(
            crate::f64::normalise(
                crate::f64::indexify_lat(-90.0) * 60.0 * 60.0,
                0.0,
                crate::f64::consts::ARC_SECONDS_IN_180_DEGREES
            ),
            crate::usize::consts::ARC_SECONDS_IN_180_DEGREES
        ),
        0
    );
    assert_eq!(
        crate::f64::normalised_to_index(
            crate::f64::normalise(
                crate::f64::indexify_lat(0.0) * 60.0 * 60.0,
                0.0,
                crate::f64::consts::ARC_SECONDS_IN_180_DEGREES
            ),
            crate::usize::consts::ARC_SECONDS_IN_180_DEGREES
        ),
        crate::usize::consts::ARC_SECONDS_IN_180_DEGREES / 2
    );
    assert_eq!(
        crate::f64::normalised_to_index(
            crate::f64::normalise(
                crate::f64::indexify_lat(90.0) * 60.0 * 60.0,
                0.0,
                crate::f64::consts::ARC_SECONDS_IN_180_DEGREES
            ),
            crate::usize::consts::ARC_SECONDS_IN_180_DEGREES
        ),
        crate::usize::consts::ARC_SECONDS_IN_180_DEGREES
    );
}

#[test]
fn test_normalised_to_index_f32() {
    assert_eq!(
        crate::f32::normalised_to_index(
            crate::f32::normalise(
                crate::f32::indexify_lat(-90.0) * 60.0 * 60.0,
                0.0,
                crate::f32::consts::ARC_SECONDS_IN_180_DEGREES
            ),
            crate::usize::consts::ARC_SECONDS_IN_180_DEGREES
        ),
        0
    );
    assert_eq!(
        crate::f32::normalised_to_index(
            crate::f32::normalise(
                crate::f32::indexify_lat(0.0) * 60.0 * 60.0,
                0.0,
                crate::f32::consts::ARC_SECONDS_IN_180_DEGREES
            ),
            crate::usize::consts::ARC_SECONDS_IN_180_DEGREES
        ),
        crate::usize::consts::ARC_SECONDS_IN_180_DEGREES / 2
    );
    assert_eq!(
        crate::f32::normalised_to_index(
            crate::f32::normalise(
                crate::f32::indexify_lat(90.0) * 60.0 * 60.0,
                0.0,
                crate::f32::consts::ARC_SECONDS_IN_180_DEGREES
            ),
            crate::usize::consts::ARC_SECONDS_IN_180_DEGREES
        ),
        crate::usize::consts::ARC_SECONDS_IN_180_DEGREES
    );
}

#[test]
fn test_indexify_lat_f64() {
    assert_eq!(crate::f64::indexify_lat(-90.0), 0.0);
    assert_eq!(crate::f64::indexify_lat(0.0), 90.0);
    assert_eq!(crate::f64::indexify_lat(90.0), 180.0);
}

#[test]
fn test_indexify_lat_f32() {
    assert_eq!(crate::f32::indexify_lat(-90.0), 0.0);
    assert_eq!(crate::f32::indexify_lat(0.0), 90.0);
    assert_eq!(crate::f32::indexify_lat(90.0), 180.0);
}

#[test]
fn test_indexify_long_f64() {
    assert_eq!(crate::f64::indexify_long(-180.0), 0.0);
    assert_eq!(crate::f64::indexify_long(0.0), 180.0);
    assert_eq!(crate::f64::indexify_long(180.0), 360.0);
}

#[test]
fn test_indexify_long_f32() {
    assert_eq!(crate::f32::indexify_long(-180.0), 0.0);
    assert_eq!(crate::f32::indexify_long(0.0), 180.0);
    assert_eq!(crate::f32::indexify_long(180.0), 360.0);
}

#[test]
fn test_indexify_lat_long_f64() {
    assert_eq!(crate::f64::indexify_lat_long(-90.0, -180.0), (0.0, 0.0));
    assert_eq!(crate::f64::indexify_lat_long(0.0, 0.0), (90.0, 180.0));
    assert_eq!(crate::f64::indexify_lat_long(90.0, 180.0), (180.0, 360.0));
}

#[test]
fn test_indexify_lat_long_f32() {
    assert_eq!(crate::f32::indexify_lat_long(-90.0, -180.0), (0.0, 0.0));
    assert_eq!(crate::f32::indexify_lat_long(0.0, 0.0), (90.0, 180.0));
    assert_eq!(crate::f32::indexify_lat_long(90.0, 180.0), (180.0, 360.0));
}

#[test]
fn test_trunc_unsafe_f64() {
    assert_eq!(crate::f64::trunc_unsafe(RAD_TO_DEG_F64, 19), RAD_TO_DEG_F64);
    assert_eq!(crate::f64::trunc_unsafe(-RAD_TO_DEG_F64, 19), -RAD_TO_DEG_F64);
    assert_eq!(crate::f64::trunc_unsafe(RAD_TO_DEG_F64, 0), 57.0);
    assert_eq!(crate::f64::trunc_unsafe(-RAD_TO_DEG_F64, 0), -57.0);
}

#[test]
#[should_panic]
fn test_trunc_unsafe_should_error_f64() {
    crate::f64::trunc_unsafe(RAD_TO_DEG_F64, 20);
    crate::f64::trunc_unsafe(-RAD_TO_DEG_F64, 20);
}

#[test]
fn test_truct_safe_f64() {
    {
        let output: Result<f64, Warning> = crate::f64::trunc_safe(RAD_TO_DEG_F64, 19);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, RAD_TO_DEG_F64);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f64, Warning> = crate::f64::trunc_safe(-RAD_TO_DEG_F64, 19);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, -RAD_TO_DEG_F64);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f64, Warning> = crate::f64::trunc_safe(RAD_TO_DEG_F64, 6);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, 57.295779);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f64, Warning> = crate::f64::trunc_safe(-RAD_TO_DEG_F64, 6);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, -57.295779);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f64, Warning> = crate::f64::trunc_safe(RAD_TO_DEG_F64, 20);
        assert!(output.is_err());
        if let Err(Warning::F64(output, message)) = output {
            assert_eq!(output, RAD_TO_DEG_F64);
            println!("Intentional warning message: \"{}\"", message);
        } else {
            panic!();
        }
    }
}

#[test]
fn test_trunc_exact_f64() {
    assert_eq!(trunc_exact(-0.2957795130823209, 18), -0.2957795130823209);
    println!("{}", crate::f64::consts::DEG_TO_RAD);
}

#[test]
#[ignore]
fn test_() {
}