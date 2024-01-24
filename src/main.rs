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

