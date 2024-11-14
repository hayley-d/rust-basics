/* 
Given a string of words, you need to find the highest scoring word.
Each letter of a word scores points according to its position in the alphabet: a = 1, b = 2, c = 3 etc.
For example, the score of abad is 8 (1 + 2 + 1 + 4).
*/
fn high(input: &str) -> &str {
    let mut max_score: u32 = 0;
    let mut result: &str = ""; 

    for word in input.split_whitespace() {
        let mut sum: u32 = 0;
        for c in word.chars() {
            sum += (c as u32) - ('a' as u32) + 1; // 'a' = 1, 'b' = 2, etc.
        }

        if sum > max_score {
            max_score = sum;
            result = word; 
        }
    }

    result
}

fn high(input: &str) -> &str {
    input
        .split_whitespace() 
        .max_by_key(|word| {
            word.chars()
                .map(|c| (c as u32) - ('a' as u32) + 1) 
                .sum::<u32>()
        })
        .unwrap_or("") 
}

fn high(input: &str) -> &str {
    input
        .split_whitespace()
        .rev() 
        .max_by_key(|word| {
            word.chars()
                .map(|c| (c as u32) - ('a' as u32) + 1)
                .sum::<u32>()
        })
        .unwrap_or("")
}
