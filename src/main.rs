use rand::Rng; // Trait that provides the random number generation methods
use std::cmp::Ordering;

const EL_COUNT: u64 = 1000000;
const VEC_COUNT: u32 = 5;

fn merge_sort(slice: &mut [f64]){
    let len = slice.len();
    if len <= 1 {return}

    let mid = len / 2;
    {
        let (first_half, second_half) = slice.split_at_mut(mid);
        merge_sort(first_half);
        merge_sort(second_half);
    }

    let mut i = 0;
    let mut j = mid;
    let mut result = vec![];
    loop {
        if !(i < mid) && !(j < len) {break}
        if !(j < len) {
            result.push(slice[i]);
            i += 1;
            continue;
        }
        if !(i < mid) {
            result.push(slice[j]);
            j += 1;
            continue;
        }
        match slice[i].partial_cmp(&slice[j]) {
            Some(Ordering::Greater) => {
                result.push(slice[j]);
                j += 1;
            },
            Some(Ordering::Less) => {
                result.push(slice[i]);
                i += 1;
            },
            Some(Ordering::Equal) => {
                result.push(slice[i]);
                i += 1;
            },
            _ => {}
        }
    }
    for (x, i) in result.into_iter().enumerate() {
        slice[x] = i;
    }
}

fn main() {
    let mut rng = rand::thread_rng(); // Get a random number generator

    let mut vec_of_vecs: Vec<Vec<f64>> = Vec::new();
    for _ in 0..VEC_COUNT {
        let nums_1: Vec<f64> = (0..EL_COUNT).map(|_| rng.gen_range(0.0..100.0)).collect();
        vec_of_vecs.push(nums_1);
    }

    let len = EL_COUNT;
    println!("Vec Len: {}", len);

    println!("Running...");

    let time_out = std::time::Instant::now();
    for (index, mut i) in vec_of_vecs.iter_mut().enumerate() {
        let time_in = std::time::Instant::now();

        merge_sort(&mut i);

        let mark_in = time_in.elapsed().as_millis();
        println!("{} in {} ms.", index, mark_in);
    }
    let mark_out = time_out.elapsed().as_millis();
    let per_mili: u64 = (VEC_COUNT as u64 * EL_COUNT as u64) / mark_out as u64;
    println!("for {} * {} vec:\n elements per mili: {}", VEC_COUNT, EL_COUNT, per_mili);
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng; // Trait that provides the random number generation methods

    const EL_COUNT: u64 = 1000000;
    const VEC_COUNT: u32 = 5;


    #[test]
    fn bench() {

        let mut rng = rand::thread_rng(); // Get a random number generator

        let mut vec_of_vecs: Vec<Vec<f64>> = Vec::new();
        for _ in 0..VEC_COUNT {
            let nums_1: Vec<f64> = (0..EL_COUNT).map(|_| rng.gen_range(0.0..100.0)).collect();
            vec_of_vecs.push(nums_1);
        }

        let len = EL_COUNT;
        println!("@ Vec Len: {}", len);

        println!("@ Running...");

        let time_out = std::time::Instant::now();
        for (index, mut i) in vec_of_vecs.iter_mut().enumerate() {
            let time_in = std::time::Instant::now();

            merge_sort(&mut i);

            let mark_in = time_in.elapsed().as_millis();
            println!("@ {} in {} ms.", index, mark_in);
        }
        let mark_out = time_out.elapsed().as_millis();
        let per_mili: u64 = (VEC_COUNT as u64 * EL_COUNT as u64) / mark_out as u64;
        println!("@ for {} * {} vec:\n@ elements per mili: {}", VEC_COUNT, EL_COUNT, per_mili);
    }
}

