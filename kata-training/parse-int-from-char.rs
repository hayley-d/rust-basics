/*
Write a program that returns the girl's age (0-9) as an integer.

Assume the test input string is always a valid string. For example, the test input may be "1 year old" or "5 years old". The first character in the string is always a number.
*/
// Version 1
fn get_age(age: &str) -> u32 {
   age.chars().next().unwrap() as u32 - '0' as u32
}

// version 2
fn get_age(age: &str) -> u32 {
    age[..1].parse().unwrap()
}
