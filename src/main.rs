use rand::Rng; // Trait that provides the random number generation methods
use std::cmp::Ordering;

const EL_COUNT: u64 = 1000000;

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
    let mut nums_1: Vec<f64> = (0..EL_COUNT).map(|_| rng.gen_range(0.0..100.0)).collect();

    let len = nums_1.len();


    println!("\nMERGE SORT");
    println!("Vec Len: {}", len);
    let time_b = std::time::Instant::now();

    merge_sort(&mut nums_1);

    let mark_fast = time_b.elapsed().as_millis();
    println!("{} ms", mark_fast);


    println!("\nMERGE VAL:");
    let len = len;
    for i in 0..len {
        if i == len - 1 {break}
        match nums_1[i].partial_cmp(&nums_1[i + 1]) {
            Some(Ordering::Greater) => print!("1:X"),
            Some(Ordering::Less) => print!(""),
            Some(Ordering::Equal) => print!("="),
            _ => {},
        }
    }

    // Size of a single element
    let size_el = std::mem::size_of::<f64>();

    // Total size of all elements
    // let total_size = size_el * len;

    // Total allocated capacity (in bytes)
    let allocated_size = size_el * nums_1.capacity();

    println!("Elements per ms: {}", len as f64 / mark_fast as f64);
    println!("KB:{}", allocated_size / 1000);
}
