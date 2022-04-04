pub mod tests {

    #[test]
    fn test_rhs_f64() {
        assert_eq!(crate::f64::rhs(57.29577951308232), 0.29577951308232286);
        assert_eq!(crate::f64::rhs(-57.29577951308232), 0.29577951308232286);
    }

    #[test]
    fn test_rhs_f32() {
        assert_eq!(crate::f32::rhs(57.29578), 0.29578018);
        assert_eq!(crate::f32::rhs(-57.29578), 0.29578018);
    }

    #[test]
    fn test_lhs_f64() {
        assert_eq!(crate::f64::lhs(57.29577951308232), 57.0);
        assert_eq!(crate::f64::lhs(-57.29577951308232), -57.0);
    }
    
    #[test]
    fn test_lhs_f32() {
        assert_eq!(crate::f32::lhs(57.29578), 57.0);
        assert_eq!(crate::f32::lhs(-57.29578), -57.0);
    }

    #[test]
    fn test_lhs_abs_f64() {
        assert_eq!(crate::f64::lhs_abs(57.29577951308232), 57.0);
        assert_eq!(crate::f64::lhs_abs(-57.29577951308232), 57.0);
    }

    #[test]
    fn test_lhs_abs_f32() {
        assert_eq!(crate::f32::lhs_abs(57.29578), 57.0);
        assert_eq!(crate::f32::lhs_abs(-57.29578), 57.0);
    }

    #[test]
    fn test_split_f64() {
        assert_eq!(crate::f64::split(57.29577951308232), (57.0, 0.29577951308232286));
        assert_eq!(crate::f64::split(-57.29577951308232), (-57.0, 0.29577951308232286));
    }

    #[test]
    fn test_split_f32() {
        assert_eq!(crate::f32::split(57.29578), (57.0, 0.29578018));
        assert_eq!(crate::f32::split(-57.29578), (-57.0, 0.29578018));
    }

    #[test]
    fn test_split_abs_f64() {
        assert_eq!(crate::f64::split_abs(57.29577951308232), (57.0, 0.29577951308232286));
        assert_eq!(crate::f64::split_abs(-57.29577951308232), (57.0, 0.29577951308232286));
    }

    #[test]
    fn test_split_abs_f32() {
        assert_eq!(crate::f32::split_abs(57.29578), (57.0, 0.29578018));
        assert_eq!(crate::f32::split_abs(-57.29578), (57.0, 0.29578018));
    }

    #[test]
    fn test_to_radians_f64() {
        let input: f64 = 100.0;//100.0000000003249
        assert_eq!(input.to_radians(), crate::f64::to_radians(input));
        assert_eq!(crate::f64::to_radians(input), 1.7453292519943295);
        assert_eq!(crate::f64::to_radians(-input), -1.7453292519943295);
    }

    #[test]
    fn test_to_radians_f32() {
        let input: f32 = 100.0;//100.0000000003249
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
        assert_eq!(crate::f64::normalise(crate::f64::consts::ARC_SECONDS_IN_360_DEGREES_F64 / 2.0, 0.0, crate::f64::consts::ARC_SECONDS_IN_360_DEGREES_F64), 0.5);
        assert_eq!(crate::f64::normalise(crate::f64::consts::ARC_SECONDS_IN_360_DEGREES_F64, 0.0, crate::f64::consts::ARC_SECONDS_IN_360_DEGREES_F64), 1.0);
        assert_eq!(crate::f64::normalise(0.0, 0.0, crate::f64::consts::ARC_SECONDS_IN_360_DEGREES_F64), 0.0);
    }

    #[test]
    fn test_normalise_f32() {
        assert_eq!(crate::f32::normalise(crate::f32::consts::ARC_SECONDS_IN_360_DEGREES_F32 / 2.0, 0.0, crate::f32::consts::ARC_SECONDS_IN_360_DEGREES_F32), 0.5);
        assert_eq!(crate::f32::normalise(crate::f32::consts::ARC_SECONDS_IN_360_DEGREES_F32, 0.0, crate::f32::consts::ARC_SECONDS_IN_360_DEGREES_F32), 1.0);
        assert_eq!(crate::f32::normalise(0.0, 0.0, crate::f32::consts::ARC_SECONDS_IN_360_DEGREES_F32), 0.0);
    }

    #[test]
    fn test_normalised_to_index_f64() {
        assert_eq!(crate::f64::normalised_to_index(crate::f64::normalise(crate::f64::indexify_lat(-90.0) * 60.0 * 60.0, 0.0, crate::f64::consts::ARC_SECONDS_IN_180_DEGREES_F64), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE), 0);
        assert_eq!(crate::f64::normalised_to_index(crate::f64::normalise(crate::f64::indexify_lat(0.0) * 60.0 * 60.0, 0.0, crate::f64::consts::ARC_SECONDS_IN_180_DEGREES_F64), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE / 2);
        assert_eq!(crate::f64::normalised_to_index(crate::f64::normalise(crate::f64::indexify_lat(90.0) * 60.0 * 60.0, 0.0, crate::f64::consts::ARC_SECONDS_IN_180_DEGREES_F64), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE);
    }

    #[test]
    fn test_normalised_to_index_f32() {
        assert_eq!(crate::f32::normalised_to_index(crate::f32::normalise(crate::f32::indexify_lat(-90.0) * 60.0 * 60.0, 0.0, crate::f32::consts::ARC_SECONDS_IN_180_DEGREES_F32), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE), 0);
        assert_eq!(crate::f32::normalised_to_index(crate::f32::normalise(crate::f32::indexify_lat(0.0) * 60.0 * 60.0, 0.0, crate::f32::consts::ARC_SECONDS_IN_180_DEGREES_F32), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE / 2);
        assert_eq!(crate::f32::normalised_to_index(crate::f32::normalise(crate::f32::indexify_lat(90.0) * 60.0 * 60.0, 0.0, crate::f32::consts::ARC_SECONDS_IN_180_DEGREES_F32), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE), crate::usize::consts::ARC_SECONDS_IN_180_DEGREES_USIZE);
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
    fn test_() {

    }
}