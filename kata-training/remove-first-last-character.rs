// Your goal is to create a function that removes the first and last characters of a string. You're given one parameter, the original string. You don't have to worry about strings with less than two characters.

pub fn remove_char(s: &str) -> String {
    if s.len() <= 2 {
        return String::new();
    }
    
    return s[1..s.len()-1].to_string();
}

//other solution
pub fn remove_char(s: &str) -> String {
    let first_last_off: &str = &s[1..s.len() - 1];
    return first_last_off.to_owned(); // to_owned better than to_string for performance
}
