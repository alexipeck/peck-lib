pub fn main() {
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;
        for i in 0..21 {
            for _ in 0..10 {
                let start = std::time::Instant::now();
                let _output = peck_lib::f64::approx_equal_infallible_f64(
                    -5.29577951308232f64,
                    -5.29577951308233f64,
                    i
                );
                total += 1;
                sum += start.elapsed().as_nanos();
            }
        }

        println!("str: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;
        for _ in 0..10 {
            let start = std::time::Instant::now();
            let _output = peck_lib::f64::approx_equal_infallible_f64(
                -5.29577951308232f64,
                -5.29577951308233f64,
                20
            );
            total += 1;
            sum += start.elapsed().as_nanos();
        }

        println!("str: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for i in 0..21 {
            for _ in 0..10 {
                let start = std::time::Instant::now();
                let _output = peck_lib::f64::approx_equal_infallible_f64(
                    -6.29577951308232f64,
                    -5.29577951308233f64,
                    i
                );
                total += 1;
                sum += start.elapsed().as_nanos();
            }
        }

        println!("str_diff_lhs(short circuits): {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for i in 0..21 {
            for _ in 0..10 {
                let start = std::time::Instant::now();
                let _output = peck_lib::f64::approx_equal_f64(
                    -6.29577951308232f64,
                    -5.29577951308233f64,
                    i
                );
                total += 1;
                sum += start.elapsed().as_nanos();
            }
        }

        println!("mathematical way(diff lhs)(diff rhs): {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for i in 0..21 {
            for _ in 0..10 {
                let start = std::time::Instant::now();
                let _output = peck_lib::f64::approx_equal_f64(
                    -5.29577951308232f64,
                    -5.29577951308233f64,
                    i
                );
                total += 1;
                sum += start.elapsed().as_nanos();
            }
        }

        println!("mathematical way(same lhs)(diff rhs): {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for i in 0..21 {
            for _ in 0..10 {
                let start = std::time::Instant::now();
                let _output = peck_lib::f64::approx_equal_infallible_f64(
                    -6.29577951308232f64,
                    -5.29577951308233f64,
                    i
                );
                total += 1;
                sum += start.elapsed().as_nanos();
            }
        }

        println!("str_diff_lhs(short circuit): {}ns", sum / total as u128);
    }
    {
        let start = std::time::Instant::now();
        for i in 0..21 {
            for _ in 0..100000 {
                let output = peck_lib::f64::approx_equal_infallible_f64(
                    -5.29577951308232f64,
                    -5.29577951308233f64,
                    i
                );
                if i <= 13 {
                    assert!(output);
                } else {
                    assert!(!output);
                }
            }
        }
        println!("str: {}ns.", start.elapsed().as_nanos());
    }
    {
        let start = std::time::Instant::now();
        for i in 0..21 {
            for _ in 0..100000 {
                let output = peck_lib::f64::approx_equal_f64(
                    -5.29577951308232f64,
                    -5.29577951308233f64,
                    i
                );
                if i <= 13 {
                    assert!(output);
                } else {
                    assert!(!output);
                }
            }
        }
        println!("math: {}ns.", start.elapsed().as_nanos());
    }
    {
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            for _ in 0..i32::MAX {
                assert_eq!(peck_lib::f64::lhs(peck_lib::f64::consts::RAD_TO_DEG), 57.0);
                assert_eq!(peck_lib::f64::lhs(-peck_lib::f64::consts::RAD_TO_DEG), -57.0);
            }
        }
        println!("peck-lhs: {}ns.", start.elapsed().as_nanos());
    }
    {
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            for _ in 0..i32::MAX {
                assert_eq!(peck_lib::f64::consts::RAD_TO_DEG.trunc(), 57.0);
                assert_eq!(-peck_lib::f64::consts::RAD_TO_DEG.trunc(), -57.0);
            }
        }
        println!("std-lhs: {}ns.", start.elapsed().as_nanos());
    }
}