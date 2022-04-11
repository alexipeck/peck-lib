pub struct Timing {
    start_time: std::time::Instant,
}

impl Timing {
    pub fn start() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }
    pub fn run(&self, _: ()) -> u128 {
        self.start_time.elapsed().as_nanos()
    }
}

pub fn main() {
    {
        let timing = Timing::start();
        let runtime = timing.run(
            std::thread::sleep(std::time::Duration::from_secs(1))
        );
        println!("Test should be 1000ms, took {}ms.", runtime / 1000000);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;
        for _ in 0..10000 {
            for _ in 0..100 {
                let start = std::time::Instant::now();
                for i in 0..21 {
                    let _output = peck_lib::f64::approx_equal_infallible_f64(
                        -5.29577951308232f64,
                        -5.29577951308233f64,
                        i
                    );
                }
                sum += start.elapsed().as_nanos();
                total += 1;
            }
        }

        println!("approx_equal_infallible_f64 * 100 string method average case (full range) timing average: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;
        for _ in 0..10000 {
            let start = std::time::Instant::now();
            for _ in 0..100 {
                let _output = peck_lib::f64::approx_equal_infallible_f64(
                    -5.29577951308232f64,
                    -5.29577951308233f64,
                    20
                );
            }
            sum += start.elapsed().as_nanos();
            total += 1;
        }

        println!("approx_equal_infallible_f64 * 100 string method worst case (max range + n) timing average: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10000 {
            let start = std::time::Instant::now();
            for _ in 0..100 {
                let _output = peck_lib::f64::approx_equal_infallible_f64(
                    -6.29577951308232f64,
                    -5.29577951308233f64,
                    0//arbitrary for this test
                );
            }
            sum += start.elapsed().as_nanos();
            total += 1;
        }

        println!("approx_equal_infallible_f64 * 100 string method best case (diff lhs, short circuit) timing average: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10000 {
            for i in 0..21 {
                let start = std::time::Instant::now();
                for _ in 0..100 {
                    let _output = peck_lib::f64::approx_equal_f64(
                        -6.29577951308232f64,
                        -5.29577951308233f64,
                        i
                    );
                }
                sum += start.elapsed().as_nanos();
                total += 1;
            }
        }

        println!("approx_equal_f64 math method (diff lhs)(diff rhs) timing average: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10000 {
            for _ in 0..100 {
                for i in 0..21 {
                    let start = std::time::Instant::now();
                    let _output = peck_lib::f64::approx_equal_f64(
                        -5.29577951308232f64,
                        -5.29577951308233f64,
                        i
                    );
                    sum += start.elapsed().as_nanos();
                }
                total += 1;
            }
        }

        println!("approx_equal_f64 math method (same lhs)(diff rhs) timing average: {}ns", sum / total as u128);
    }
    {   
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10000 {
            for _ in 0..100 {
                for i in 0..21 {
                    let start = std::time::Instant::now();
                    let _output = peck_lib::f64::approx_equal_f64(
                        -5.29577951308232f64,
                        -5.29577951308232f64,
                        i
                    );
                    sum += start.elapsed().as_nanos();
            }
            total += 1;
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
        
        for _ in 0..10000 {
            for _ in 0..100 {
                let start = std::time::Instant::now();
                let _output: f64 = peck_lib::f64::lhs(peck_lib::f64::consts::RAD_TO_DEG);
                sum += start.elapsed().as_nanos();
            }
            total += 1;
        }
        println!("peck-lhs: {}ns.", sum / total as u128);
    }
    {
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10000 {
            for _ in 0..100 {
                let start = std::time::Instant::now();
                let _output: f64 = peck_lib::f64::consts::RAD_TO_DEG.trunc();
                sum += start.elapsed().as_nanos();
            }
            total += 1;
        }
        println!("std-lhs: {}ns.", sum / total as u128);
    }
    {
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10000 {
            for _ in 0..100 {
                let start = std::time::Instant::now();
                let _output: f64 = peck_lib::f64::to_radians(100.0f64);
                sum += start.elapsed().as_nanos();
            }
            total += 1;
        }
        println!("peck_lib::f64::to_radians * 100: {}ns.", sum / total as u128);
    }
    {
        let mut sum: u128 = 0u128;
        let mut total: usize = 0;

        for _ in 0..10000 {
            for _ in 0..100 {
                let start = std::time::Instant::now();
                let _output: f64 = 100.0f64.to_radians();
                sum += start.elapsed().as_nanos();
            }
            total += 1;
        }
        println!("std::f64::to_radians: {}ns.", sum / total as u128);
    }
}