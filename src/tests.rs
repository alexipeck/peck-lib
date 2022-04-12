#[test]
fn test_split_f64() {
    {
        let (lhs, rhs) = crate::f64::split(crate::f64::consts::RAD_TO_DEG);
        assert_eq!(lhs, 57.0);
        assert!(crate::f64::approx_equal_f64(rhs, 0.29577951308232, 13));
    }
    {
        let (lhs, rhs) = crate::f64::split(-crate::f64::consts::RAD_TO_DEG);
        assert_eq!(lhs, -57.0);
        println!("rhs: {}", rhs);
        assert!(crate::f64::approx_equal_f64(rhs, -0.29577951308232, 13));
    }
}

#[test]
fn test_split_f32() {
    {
        let (lhs, rhs) = crate::f32::split(crate::f32::consts::RAD_TO_DEG);
        assert_eq!(lhs, 57.0);
        assert!(crate::f32::approx_equal_f32(rhs, 0.29578, 5));
    }
    {
        let (lhs, rhs) = crate::f32::split(-crate::f32::consts::RAD_TO_DEG);
        assert_eq!(lhs, -57.0);
        assert!(crate::f32::approx_equal_f32(rhs, -0.29578, 5));
    }
}

#[test]
fn test_split_abs_f64() {
    {
        let (lhs, rhs) = crate::f64::split_abs(crate::f64::consts::RAD_TO_DEG);
        assert_eq!(lhs, 57.0);
        assert!(crate::f64::approx_equal_f64(rhs, 0.29577951308232, 13));
    }
    {
        let (lhs, rhs) = crate::f64::split_abs(-crate::f64::consts::RAD_TO_DEG);
        assert_eq!(lhs, 57.0);
        println!("{}", rhs);
        assert!(crate::f64::approx_equal_f64(rhs, 0.29577951308232, 13));
    }
}

#[test]
fn test_split_abs_f32() {
    {
        let (lhs, rhs) = crate::f32::split_abs(crate::f32::consts::RAD_TO_DEG);
        assert_eq!(lhs, 57.0);
        assert!(crate::f32::approx_equal_f32(rhs, 0.29578, 5));
    }
    {
        let (lhs, rhs) = crate::f32::split_abs(-crate::f32::consts::RAD_TO_DEG);
        assert_eq!(lhs, 57.0);
        assert!(crate::f32::approx_equal_f32(rhs, 0.29578, 5));
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
    assert_eq!(crate::f32::to_radians(input), input.to_radians());
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
fn test_trunc_f64() {
    assert_eq!(
        crate::f64::trunc(crate::f64::consts::RAD_TO_DEG, 19),
        crate::f64::consts::RAD_TO_DEG
    );
    assert_eq!(
        crate::f64::trunc(-crate::f64::consts::RAD_TO_DEG, 19),
        -crate::f64::consts::RAD_TO_DEG
    );
    assert_eq!(crate::f64::trunc(crate::f64::consts::RAD_TO_DEG, 0), 57.0);
    assert_eq!(crate::f64::trunc(-crate::f64::consts::RAD_TO_DEG, 0), -57.0);
}

#[test]
fn test_trunc_f32() {
    assert_eq!(
        crate::f32::trunc(crate::f32::consts::RAD_TO_DEG, 19),
        crate::f32::consts::RAD_TO_DEG
    );
    assert_eq!(
        crate::f32::trunc(-crate::f32::consts::RAD_TO_DEG, 19),
        -crate::f32::consts::RAD_TO_DEG
    );
    assert_eq!(crate::f32::trunc(crate::f32::consts::RAD_TO_DEG, 0), 57.0);
    assert_eq!(crate::f32::trunc(-crate::f32::consts::RAD_TO_DEG, 0), -57.0);
}

#[test]
#[should_panic]
fn test_trunc_f64_should_error() {
    crate::f64::trunc(crate::f64::consts::RAD_TO_DEG, 20);
    crate::f64::trunc(-crate::f64::consts::RAD_TO_DEG, 20);
}

#[test]
#[should_panic]
fn test_trunc_f32_should_error() {
    crate::f32::trunc(crate::f32::consts::RAD_TO_DEG, 20);
    crate::f32::trunc(-crate::f32::consts::RAD_TO_DEG, 20);
}

#[test]
fn test_truct_safe_f64() {
    {
        let output: Result<f64, crate::error::Warning> =
            crate::f64::trunc_safe(crate::f64::consts::RAD_TO_DEG, 19);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, crate::f64::consts::RAD_TO_DEG);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f64, crate::error::Warning> =
            crate::f64::trunc_safe(-crate::f64::consts::RAD_TO_DEG, 19);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, -crate::f64::consts::RAD_TO_DEG);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f64, crate::error::Warning> =
            crate::f64::trunc_safe(crate::f64::consts::RAD_TO_DEG, 6);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, 57.295779);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f64, crate::error::Warning> =
            crate::f64::trunc_safe(-crate::f64::consts::RAD_TO_DEG, 6);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, -57.295779);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f64, crate::error::Warning> =
            crate::f64::trunc_safe(crate::f64::consts::RAD_TO_DEG, 20);
        assert!(output.is_err());
        if let Err(crate::error::Warning::F64(output, message)) = output {
            assert_eq!(output, crate::f64::consts::RAD_TO_DEG);
            println!("Intentional warning message: \"{}\"", message);
        } else {
            panic!();
        }
    }
}

#[test]
fn test_truct_safe_f32() {
    {
        let output: Result<f32, crate::error::Warning> =
            crate::f32::trunc_safe(crate::f32::consts::RAD_TO_DEG, 19);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, crate::f32::consts::RAD_TO_DEG);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f32, crate::error::Warning> =
            crate::f32::trunc_safe(-crate::f32::consts::RAD_TO_DEG, 19);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, -crate::f32::consts::RAD_TO_DEG);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f32, crate::error::Warning> =
            crate::f32::trunc_safe(crate::f32::consts::RAD_TO_DEG, 4);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, 57.2957);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f32, crate::error::Warning> =
            crate::f32::trunc_safe(-crate::f32::consts::RAD_TO_DEG, 4);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert_eq!(output, -57.2957);
        } else {
            panic!();
        }
    }
    {
        let output: Result<f32, crate::error::Warning> =
            crate::f32::trunc_safe(crate::f32::consts::RAD_TO_DEG, 20);
        assert!(output.is_err());
        if let Err(crate::error::Warning::F32(output, message)) = output {
            assert_eq!(output, crate::f32::consts::RAD_TO_DEG);
            println!("Intentional warning message: \"{}\"", message);
        } else {
            panic!();
        }
    }
}

#[test]
fn test_trunc_exact_f64() {
    assert_eq!(
        crate::f64::trunc_exact(-0.2957795130823209, 18),
        -0.2957795130823209
    );
    assert_eq!(
        crate::f64::trunc_exact(-0.2957795130823209, 16),
        -0.2957795130823209
    );
    assert_eq!(
        crate::f64::trunc_exact(-0.2957795130823209, 14),
        -0.29577951308232
    );
}

#[test]
fn test_trunc_exact_f32() {
    assert_eq!(crate::f32::trunc_exact(-0.29577953, 8), -0.29577953);
    assert_eq!(crate::f32::trunc_exact(-0.29577953, 6), -0.295779);
    assert_eq!(crate::f32::trunc_exact(-0.29577953, 4), -0.2957);
}

#[test]
fn test_two_times_pi_equals_pi_plus_pi_f64() {
    assert_eq!(
        std::f64::consts::PI + std::f64::consts::PI,
        2.0f64 * std::f64::consts::PI
    );
}

#[test]
fn test_two_times_pi_equals_pi_plus_pi_f64_to_f32() {
    assert_eq!(
        (std::f64::consts::PI + std::f64::consts::PI) as f32,
        (2.0f64 * std::f64::consts::PI) as f32
    );
}

#[test]
fn test_two_times_pi_equals_pi_plus_pi_f32() {
    assert_eq!(
        std::f32::consts::PI + std::f32::consts::PI,
        2.0f32 * std::f32::consts::PI
    );
}

#[test]
fn test_rad_to_deg_f64() {
    assert_eq!(
        crate::f64::consts::RAD_TO_DEG,
        180.0f64 / std::f64::consts::PI
    );
}

#[test]
fn test_rad_to_deg_f32() {
    assert!(crate::f32::approx_equal_f32(
        crate::f32::consts::RAD_TO_DEG,
        180.0f32 / std::f32::consts::PI,
        4
    ));
}

#[test]
fn test_deg_to_rad_f64() {
    assert_eq!(
        crate::f64::consts::DEG_TO_RAD,
        std::f64::consts::PI / 180.0f64
    );
}

#[test]
fn test_deg_to_rad_f32() {
    assert_eq!(
        crate::f32::consts::DEG_TO_RAD,
        std::f32::consts::PI / 180.0f32
    );
}

#[test]
fn test_approx_equal_f64() {
    assert!(crate::f64::approx_equal_f64(
        -5.29577951308232f64,
        -5.29577951308233f64,
        13
    ));
    assert!(!crate::f64::approx_equal_f64(
        -5.29577951308232f64,
        -5.29577951308233f64,
        14
    ));
}

#[test]
#[should_panic]
fn test_approx_equal_f64_should_error() {
    let _ = crate::f64::approx_equal_f64(-5.29577951308232f64, -5.29577951308232f64, 20);
}

#[test]
fn test_approx_equal_infallible_f64() {
    assert!(crate::f64::approx_equal_infallible_f64(
        -5.29577951308232f64,
        -5.29577951308233f64,
        13
    ));
    assert!(!crate::f64::approx_equal_infallible_f64(
        -5.29577951308232f64,
        -5.29577951308233f64,
        14
    ));
    assert!(!crate::f64::approx_equal_infallible_f64(
        -5.29577951308232f64,
        -5.29577951308233f64,
        20
    ));
    assert!(crate::f64::approx_equal_infallible_f64(
        -5.29577951308232f64,
        -5.29577951308232f64,
        20
    ));
}

#[test]
fn test_trunc_str() {
    assert_eq!(crate::str::trunc("0123456789", 0), "");
    assert_eq!(crate::str::trunc("0123456789", 1), "0");
    assert_eq!(crate::str::trunc("0123456789", 5), "01234");
    assert_eq!(crate::str::trunc("0123456789", 10), "0123456789");
    assert_eq!(crate::str::trunc("0123456789", 11), "0123456789");
}

#[test]
#[ignore]
fn test_() {}
