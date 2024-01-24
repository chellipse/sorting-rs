
#[cfg(test)]
mod tests {
    use rand::Rng; // Trait that provides the random number generation methods
    use merge::Sortable; // Trait to be tested

    // a macro for producing functions which generate vectors of test data
    macro_rules! make_rand_func {
        ($funcname:ident, $type:ty, $count:expr, $start:expr, $end:expr) => {
            fn $funcname() -> Vec<$type> {
                let mut rng = rand::thread_rng(); // Get a random number generator
                let res: Vec<$type> = (0..$count)
                                            .map(|_| rng.gen_range($start..$end))
                                            .collect();
                res
            }
        }
    }

    // ONLY VALID FOR TYPES THAT HAVE THE Ord TRAIT
    // (ie, don't use this for a float)
    macro_rules! make_test_ord {
        ($funcname:ident, $type:ty, $start:expr, $end:expr) => {
            #[test]
            fn $funcname() {
                make_rand_func!(make_vec, $type, 1000000, $start, $end);
                let mut data: Vec<$type> = make_vec();
                let time = std::time::Instant::now();
                (&mut data).sort_merge();
                println!("Sort time: {}ms", time.elapsed().as_millis());
                for i in 0..data.len() {
                    if (i+1) >= data.len() {
                        break
                    }
                    assert!(data[i] <= data[i+1]);
                }
            }
        }
    }

    // use this one for floats, it will Break if a NaN value is run into
    // the f32/64 sort_merge() impl will place all NaN values at the end
    // so breaking when we meet one is ok
    macro_rules! make_test_float {
        ($funcname:ident, $type:ty, $start:expr, $end:expr) => {
            #[test]
            fn $funcname() {
                make_rand_func!(make_vec, $type, 1000000, $start, $end);
                let mut data: Vec<$type> = make_vec();
                let time = std::time::Instant::now();
                (&mut data).sort_merge();
                println!("Sort time: {}ms", time.elapsed().as_millis());
                for i in 0..data.len() {
                    if (i+1) >= data.len() {
                        break
                    } else
                    if data[i+1].is_nan() {
                        break
                    }
                    assert!(data[i] <= data[i+1]);
                }
            }
        }
    }

    // unsigned
    make_test_ord!(test_u8, u8, 0, 255);
    make_test_ord!(test_u16, u16, 0, 65535);
    make_test_ord!(test_u32, u32, 0, 4294967295);
    make_test_ord!(test_u64, u64, 0, 18446744073709551615);
    make_test_ord!(test_u128, u128, 0, 340282366920938463463374607431768211455);

    // signed
    make_test_ord!(test_i8, i8, -127, 127);
    make_test_ord!(test_i16, i16, -32767, 32767);
    make_test_ord!(test_i32, i32, -2147483647, 2147483647);
    make_test_ord!(test_i64, i64, -9223372036854775807_i64, 9223372036854775807);
    make_test_ord!(test_i128, i128, -170141183460469231731687303715884105727_i128, 170141183460469231731687303715884105727);

    // float
    make_test_float!(test_f32, f32, -3201.3452893840, 2339.38981340);
    make_test_float!(test_f64, f64, -383882.59298482010, 9629482273.3182949581390);
}

