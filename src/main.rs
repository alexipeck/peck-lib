pub fn main() {
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10 {
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

        println!("str: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10 {
            for i in 0..21 {
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

        println!("str_diff_lhs: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10 {
            for i in 0..21 {
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

        println!("math: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10 {
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

        println!("str_diff_lhs: {}ns", sum / total as u128);
    }
    {
        let start = std::time::Instant::now();
        for i in 0..21 {
            println!("{}", i);
            for _ in 0..1000000 {
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
        println!("Benchmark ran 20 million times and took {}ms.", start.elapsed().as_millis());
    }
    {
        let start = std::time::Instant::now();
        for i in 0..21 {
            println!("{}", i);
            for _ in 0..1000000 {
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
        println!("Benchmark ran 20 million times and took {}ms.", start.elapsed().as_millis());
    }
}