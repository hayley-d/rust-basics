/*
You get an array of numbers, return the sum of all of the positives ones.

Example [1,-4,7,12] => 1 + 7 + 12 = 20

Note: if there is nothing to sum, the sum is default to 0.

*/
fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|&&x| x > 0).sum::<i32>()
}

/*
Given an array of integers, remove the smallest value. Do not mutate the original array/list. If there are multiple elements with the same value, remove the one with the lowest index. 
If you get an empty array/list, return an empty array/list.
*/
fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    if let Some((min_index, _)) = numbers.iter().enumerate().min_by_key(|&(_, &x)| x) {
        numbers.iter().enumerate()
            .filter_map(|(i, &x)| if i != min_index { Some(x) } else { None })
            .collect()
    } else {
        Vec::new()
    }
}
