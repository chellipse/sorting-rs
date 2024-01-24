use std::cmp::Ordering;

// func for types which can be purely compared
// ie comparisons must result in one of: <, >, or ==
fn merge_sort_ord<T: Ord + Copy>(slice: &mut [T]){
    let len = slice.len();
    // stops endless recursive calls
    if len <= 1 {return}

    // split our slice *evenly and recursively call
    let mid = len / 2;
    {
        let (first_half, second_half) = slice.split_at_mut(mid);
        merge_sort_ord(first_half);
        merge_sort_ord(second_half);
    }

    // vec to store our sorted list before overwriting the original
    let mut result = vec![];
    // variables to track as we iterate
    let mut y = 0;
    let mut x = mid;
    loop {
        // if x is depleated of elements, push remaining from y
        if x >= len {
            while y < mid {
                result.push(slice[y]);
                y += 1;
            }
            break
        }
        // if y is depleated of elements, push remaining from x
        if y >= mid {
            while x < mid {
                result.push(slice[x]);
                x += 1;
            }
            break
        }
        // push value based on comparison
        match slice[y].cmp(&slice[x]) {
            Ordering::Greater => {
                result.push(slice[x]);
                x += 1;
            },
            Ordering::Less => {
                result.push(slice[y]);
                y += 1;
            },
            Ordering::Equal => {
                result.push(slice[y]);
                y += 1;
            },
        }
    }
    // overwrite original vec
    for (i, e) in result.into_iter().enumerate() {
        slice[i] = e;
    }
}

// Define a macro to create functions for f32 and f64
// this is because NaN is a possible f32/f64.
// When either element is a NaN partial_cmp() will return None.
macro_rules! make_float_sort_func {
    ($func_name:ident, $type:ident) => {
        fn $func_name(slice: &mut [$type]) {
            let len = slice.len();
            // stops endless recursive calls
            if len <= 1 {return}

            // split our slice *evenly and recursively call
            let mid = len / 2;
            {
                let (first_half, second_half) = slice.split_at_mut(mid);
                $func_name(first_half);
                $func_name(second_half);
            }

            // vec to store our sorted list before overwriting the original
            let mut result = vec![];
            // variables to track as we iterate
            let mut y = 0;
            let mut x = mid;
            // count for NaN values
            let mut nan_count = 0;
            loop {
                // if x is depleated of elements, push remaining from y
                if x >= len {
                    while y < mid {
                        if slice[y].is_nan() {
                            nan_count += 1;
                        } else {
                            result.push(slice[y]);
                        }
                        y += 1;
                    }
                    break
                }
                // if y is depleated of elements, push remaining from x
                if y >= mid {
                    while x < mid {
                        if slice[x].is_nan() {
                            nan_count += 1;
                        } else {
                            result.push(slice[x]);
                        }
                        x += 1;
                    }
                    break
                }
                // push value based on comparison
                match slice[y].partial_cmp(&slice[x]) {
                    Some(Ordering::Greater) => {
                        result.push(slice[x]);
                        x += 1;
                    },
                    Some(Ordering::Less) => {
                        result.push(slice[y]);
                        y += 1;
                    },
                    Some(Ordering::Equal) => {
                        result.push(slice[y]);
                        y += 1;
                    },
                    _ => { // increment NaN count for each NaN
                        if slice[y].is_nan() {
                            nan_count += 1;
                            y += 1;
                        }
                        if slice[x].is_nan() {
                            nan_count += 1;
                            x += 1;
                        }
                    }
                }
            }
            // push NaN values to the end of $result
            for _ in 1..=nan_count {
                result.push(std::$type::NAN);
            }
            // overwrite original vec
            for (index, element) in result.into_iter().enumerate() {
                slice[index] = element;
            }
        }
    }
}

// trait which enables method use
pub trait Sortable {
    fn sort_merge(&mut self);
}

// Use the macro to define a function
make_float_sort_func!(merge_sort_f32, f32);
make_float_sort_func!(merge_sort_f64, f64);

// create a implementation purely comparable types
// cc: merge_sort_ord comment
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

// signed
impl_merge_for_type!(i8);
impl_merge_for_type!(i16);
impl_merge_for_type!(i32);
impl_merge_for_type!(i64);
impl_merge_for_type!(i128);

// impl method for f32
impl Sortable for &mut Vec<f32> {
    fn sort_merge(&mut self) {
        merge_sort_f32(self)
    }
}

// impl method for f32
impl Sortable for &mut Vec<f64> {
    fn sort_merge(&mut self) {
        merge_sort_f64(self)
    }
}

