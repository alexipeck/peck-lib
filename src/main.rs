pub fn main() {
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;
        for _ in 0..1000000 {
            for i in 0..21 {
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

        println!("approx_equal_infallible_f64 string method average case (full range) timing average: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;
        for _ in 0..1000000 {
            let start = std::time::Instant::now();
            let _output = peck_lib::f64::approx_equal_infallible_f64(
                -5.29577951308232f64,
                -5.29577951308233f64,
                20
            );
            total += 1;
            sum += start.elapsed().as_nanos();
        }

        println!("approx_equal_infallible_f64 string method worst case (max range + n) timing average: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..1000000 {
            let start = std::time::Instant::now();
            let _output = peck_lib::f64::approx_equal_infallible_f64(
                -6.29577951308232f64,
                -5.29577951308233f64,
                0//arbitrary for this test
            );
            total += 1;
            sum += start.elapsed().as_nanos();
        }

        println!("approx_equal_infallible_f64 string method best case (diff lhs, short circuit) timing average: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..1000000 {
            for i in 0..21 {
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

        println!("approx_equal_f64 math method (diff lhs)(diff rhs) timing average: {}ns", sum / total as u128);
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

        println!("approx_equal_f64 math method (same lhs)(diff rhs) timing average: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..1000000 {
            for i in 0..21 {
                let start = std::time::Instant::now();
                let _output = peck_lib::f64::approx_equal_f64(
                    -5.29577951308232f64,
                    -5.29577951308232f64,
                    i
                );
                total += 1;
                sum += start.elapsed().as_nanos();
            }
        }

        println!("approx_equal_f64 math method (same lhs)(same rhs) timing average: {}ns", sum / total as u128);
    }
    /* {   
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
    } */
    /* {
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
    } */
    /* {
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
    } */
    {
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;
        
        for _ in 0..i32::MAX {
            let start = std::time::Instant::now();
            let _output: f64 = peck_lib::f64::lhs(peck_lib::f64::consts::RAD_TO_DEG);
            total += 1;
            sum += start.elapsed().as_nanos();
        }
        println!("peck-lhs: {}ns.", sum / total as u128);
    }
    {
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        let start = std::time::Instant::now();
        for _ in 0..i32::MAX {
            let _output = peck_lib::f64::consts::RAD_TO_DEG.trunc();
            total += 1;
            sum += start.elapsed().as_nanos();
        }
        println!("std-lhs: {}ns.", sum / total as u128);
    }
}