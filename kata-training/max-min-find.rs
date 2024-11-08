/*
Your task is to make two functions ( max and min, or maximum and minimum, etc., depending on the language ) that receive a list of integers as input, 
and return the largest and lowest number in that list, respectively. Each function returns one number.
*/

// Version 1
fn minimum(arr: &[i32]) -> i32 {
    arr.iter().min().map(|&min| min).unwrap()
}

fn maximum(arr: &[i32]) -> i32 {
    arr.iter().max().map(|&max| max).unwrap()
}

// Version 2
fn minimum_v2(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

fn maximum_v2(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap()
}

// Version 3
fn minimum_v3(arr: &[i32]) -> i32 {
    match arr.iter().min() {
        Some(&min) => min,
        None => 0,
    }
}

fn maximum_v3(arr: &[i32]) -> i32 {
    match arr.iter().max() {
        Some(&max) => max,
        None => 0,
    }
}
