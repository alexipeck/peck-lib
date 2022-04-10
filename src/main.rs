pub fn main() {
    {
        let start = std::time::Instant::now();
        for i in 0..21 {
            println!("{}", i);
            for _ in 0..1000000 {
                let output = peck_lib::f64::approx_equal_f64_str(
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