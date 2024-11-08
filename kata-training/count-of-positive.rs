/*Given an array of integers.

Return an array, where the first element is the count of positives numbers and the second element is sum of negative numbers. 0 is neither positive nor negative.

If the input is an empty array or is null, return an empty array.
*/

// Version 1
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if  input.len() == 0 {return Vec::new();}
    let mut result : Vec<i32> = vec![0,0];
    
    for num in input {
        if num>0 {
            result[0] += 1;
        } else {
            result[1] += num;
        }
    }
    
    result 
}

// Version 2
fn count_positives_sum_negatives_v2(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return Vec::new();
    }
    
    vec![
        input.iter().filter(|&&x| x > 0).count() as i32,
        input.iter().filter(|&&x| x < 0).sum()
    ]
}

// Version 3
fn count_positives_sum_negatives_v3(input: Vec<i32>) -> Vec<i32> {
    match input.len() {
        0 => vec![],
        _ => vec![
            input.iter().filter(|&&x| x > 0).count() as i32,
            input.iter().filter(|&&x| x < 0).sum(),
        ],
    }
}

