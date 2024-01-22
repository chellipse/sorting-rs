use std::cmp::Ordering;

fn merge_sort_ord<T: Ord + Copy>(slice: &mut [T]){
    let len = slice.len();
    if len <= 1 {return}

    let mid = len / 2;
    {
        let (first_half, second_half) = slice.split_at_mut(mid);
        merge_sort_ord(first_half);
        merge_sort_ord(second_half);
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
        match slice[i].cmp(&slice[j]) {
            Ordering::Greater => {
                result.push(slice[j]);
                j += 1;
            },
            Ordering::Less => {
                result.push(slice[i]);
                i += 1;
            },
            Ordering::Equal => {
                result.push(slice[i]);
                i += 1;
            },
        }
    }
    for (x, i) in result.into_iter().enumerate() {
        slice[x] = i;
    }
}

// Define a macro to create functions for f32 and f64
// this is because NaN is a possible f32/f64.
// When either element is a NaN partial_cmp() will return None.
macro_rules! make_float_sort_func {
    ($func_name:ident, $type:ident) => {
        fn $func_name(slice: &mut [$type]) {
            let len = slice.len();
            if len <= 1 {return}

            let mid = len / 2;
            {
                let (first_half, second_half) = slice.split_at_mut(mid);
                $func_name(first_half);
                $func_name(second_half);
            }

            let mut i = 0;
            let mut j = mid;
            let mut result = vec![];
            let mut nan_count = 0;
            loop {
                if !(i < mid) && !(j < len) {break}
                if !(j < len) {
                    if slice[i].is_nan() {
                        nan_count += 1;
                    } else {
                        result.push(slice[i]);
                    }
                    i += 1;
                    continue;
                }
                if !(i < mid) {
                    if slice[j].is_nan() {
                        nan_count += 1;
                    } else {
                        result.push(slice[j]);
                    }
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
                    _ => {
                        if slice[i].is_nan() {
                            i += 1;
                        }
                        if slice[j].is_nan() {
                            j += 1;
                        }
                    }
                }
            }
            for _ in 0..=nan_count {
                result.push(std::$type::NAN);
            }
            // println!("NAN: {:?}", nan_count);
            // println!("RES: {:?}", result);
            println!("{}", slice.len());
            println!("{}", result.len());
            for (index, element) in result.into_iter().enumerate() {
                // println!("{element}");
                // println!("{}", slice.len());
                // println!("{}", index);
                slice[index] = element;
            }
        }
    }
}

pub trait Sortable {
    fn sort_merge(&mut self);
}

// Use the macro to define a function
make_float_sort_func!(merge_sort_f32, f32);
make_float_sort_func!(merge_sort_f64, f64);

macro_rules! impl_merge_for_type {
    ($type:ident) => {
        impl Sortable for &mut Vec<$type> {
            fn sort_merge(&mut self) {
                merge_sort_ord(self)
            }
        }
    }
}

// unsigned
impl_merge_for_type!(u8);
impl_merge_for_type!(u16);
impl_merge_for_type!(u32);
impl_merge_for_type!(u64);
impl_merge_for_type!(u128);

// floating point
impl Sortable for &mut Vec<f32> {
    fn sort_merge(&mut self) {
        merge_sort_f32(self)
    }
}

impl Sortable for &mut Vec<f64> {
    fn sort_merge(&mut self) {
        merge_sort_f64(self)
    }
}

