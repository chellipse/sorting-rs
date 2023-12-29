use rand::Rng; // Trait that provides the random number generation methods
use std::cmp::Ordering;

static mut COUNTER: u64 = 0;
static mut COUNTER_INPLACE: u64 = 0;

fn merge_sort(slice: &mut [f64]){
    unsafe {
        COUNTER += 1;
    }

    let len = slice.len();
    if len <= 1 {
        return
    }

    let mid = len / 2;
    // not supposed to be slices, idk fix this
    {
        let (first_half, second_half) = slice.split_at_mut(mid);
        merge_sort(first_half);
        merge_sort(second_half);
    }
    // In-place merge
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

fn merge_sort_inplace(slice: &mut [f64]) {
    unsafe {
        COUNTER_INPLACE += 1;
    }

    let len = slice.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    {
        let (first_half, second_half) = slice.split_at_mut(mid);
        merge_sort_inplace(first_half);
        merge_sort_inplace(second_half);
    }
    // In-place merge
    let mut i = 0;
    let mut j = mid;

    loop {
        if i == j || !(j < len) {break}
        match slice[i].partial_cmp(&slice[j]) {
            Some(Ordering::Greater) => {
                let el_j = slice[j];
                for x in (i..=j).rev() {
                    if x == 0 {break};
                    slice[x] = slice[x - 1];
                }
                slice[i] = el_j;
                i += 1;
                j += 1;
            },
            Some(Ordering::Less) => {
                i += 1;
            },
            Some(Ordering::Equal) => {
                i += 1;
            },
            _ => {}
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng(); // Get a random number generator
    let mut nums_1: Vec<f64> = (0..10000000).map(|_| rng.gen_range(0.0..100.0)).collect();
    let mut nums_2 = nums_1.clone();


    println!("\nMERGE SORT");
    let time_b = std::time::Instant::now();
    merge_sort(&mut nums_2);
    println!("{} ms", time_b.elapsed().as_millis());
    let mark_fast = time_b.elapsed().as_micros();
    println!("{} μs", mark_fast);

    println!("");
    unsafe {
        println!("CALL COUNT: {}", COUNTER);
    }

    println!("\nIN PLACE");
    let time_a = std::time::Instant::now();
    // merge_sort_inplace(&mut nums_1);
    println!("{} ms", time_a.elapsed().as_millis());
    let mark_inplace = time_a.elapsed().as_micros();
    println!("{} μs", mark_inplace);

    println!("");
    unsafe {
        println!("CALL COUNT: {}", COUNTER_INPLACE);
    }


    println!("\nMERGE:");
    let len = nums_2.len();
    for i in 0..len {
        if i == len - 1 {break}
        match nums_2[i].partial_cmp(&nums_2[i + 1]) {
            Some(Ordering::Greater) => print!("1:X"),
            Some(Ordering::Less) => print!(""),
            Some(Ordering::Equal) => print!("="),
            _ => {},
        }
    }

    println!("\nMERGE IN-PLACE:");
    // let len = nums_1.len();
    // for i in 0..len {
    //     if i == len - 1 {break}
    //     match nums_1[i].partial_cmp(&nums_1[i + 1]) {
    //         Some(Ordering::Greater) => print!("2:X"),
    //         Some(Ordering::Less) => print!(""),
    //         Some(Ordering::Equal) => print!("="),
    //         _ => {},
    //     }
    // }

    // println!("Time ratio of in-place/merge = {:.8}", mark_inplace as f64 / mark_fast as f64);

    // println!("Elements per ms: {} (inplace)", (nums_2.len() as f64 / mark_inplace as f64) * 1000.0);
    println!("Elements per ms: {}", (nums_2.len() as f64 / mark_fast as f64) * 1000.0);
}
