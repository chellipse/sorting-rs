// use rand::Rng; // Trait that provides the random number generation methods

// mod lib;
use merge::Sortable;

// const EL_COUNT: u64 = 1000000;
// const VEC_COUNT: u32 = 5;

fn main() {
    // let mut rng = rand::thread_rng(); // Get a random number generator

    // let mut vec_of_vecs: Vec<Vec<u32>> = Vec::new();
    // for _ in 0..VEC_COUNT {
    //     let nums_1: Vec<u32> = (0..EL_COUNT).map(|_| rng.gen_range(0..100)).collect();
    //     vec_of_vecs.push(nums_1);
    // }

    let mut vec_floats = vec![1.3, 1.1, std::f64::NAN, 1.5, std::f64::NAN, 1.2, std::f64::NAN, 1.4];
    // let mut vec_floats = vec![3.2, 1.1, 2.7, 10.0, 4.0, 10.0, 1.8, 2.0, 10.0, 4.2];
    println!("{:?}", vec_floats);
    (&mut vec_floats).sort_merge();
    println!("{:?}", vec_floats);

    // let len = EL_COUNT;
    // println!("Vec Len: {}", len);

    // println!("Running...");

    // let time_out = std::time::Instant::now();
    // for (index, mut i) in vec_of_vecs.iter_mut().enumerate() {
    //     let time_in = std::time::Instant::now();

    //     i.sort_merge();

    //     let mark_in = time_in.elapsed().as_millis();
    //     println!("{} in {} ms.", index, mark_in);
    // }
    // let mark_out = time_out.elapsed().as_millis();
    // let per_mili: u64 = (VEC_COUNT as u64 * EL_COUNT as u64) / mark_out as u64;
    // println!("for {} * {} vec:\n elements per mili: {}", VEC_COUNT, EL_COUNT, per_mili);
}


#[cfg(test)]
mod tests {
    // use super::*;
    use rand::Rng; // Trait that provides the random number generation methods
    use merge::Sortable;

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
                (&mut data).sort_merge();
                for i in 0..data.len() {
                    if (i+1) >= data.len() {
                        break
                    }
                    assert!(data[i] <= data[i+1]);
                }
                println!("LEN: {}", data.len());
                println!("Range: {} - {}", $start, $end);
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

    // #[test]
    // fn bench() {

    //     let mut rng = rand::thread_rng(); // Get a random number generator

    //     let mut vec_of_vecs: Vec<Vec<f64>> = Vec::new();
    //     for _ in 0..VEC_COUNT {
    //         let nums_1: Vec<f64> = (0..EL_COUNT).map(|_| rng.gen_range(0.0..100.0)).collect();
    //         vec_of_vecs.push(nums_1);
    //     }

    //     let len = EL_COUNT;
    //     println!("@ Vec Len: {}", len);

    //     println!("@ Running...");

    //     let time_out = std::time::Instant::now();
    //     for (index, mut i) in vec_of_vecs.iter_mut().enumerate() {
    //         let time_in = std::time::Instant::now();

    //         merge_sort(&mut i);

    //         let mark_in = time_in.elapsed().as_millis();
    //         println!("@ {} in {} ms.", index, mark_in);
    //     }
    //     let mark_out = time_out.elapsed().as_millis();
    //     let per_mili: u64 = (VEC_COUNT as u64 * EL_COUNT as u64) / mark_out as u64;
    //     println!("@ for {} * {} vec:\n@ elements per mili: {}", VEC_COUNT, EL_COUNT, per_mili);
    // }
}

