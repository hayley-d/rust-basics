//Write function RemoveExclamationMarks which removes all exclamation marks from a given string.
// Version 1
fn remove_exclamation_marks(input: &str) -> String {
    return input.replace("!","");
}

fn remove_exclamation_marks(input: &str) -> String {
    input.chars().filter(|&c| c != '!').collect()
}

//  'input.chars()' iterates over the characters of the input string, 'filter(|&c| c != '!')' filters out
//  the exclamation marks, and 'collect()' gathers the remaining characters into a new 'String'.

fn remove_exclamation_marks(input: &str) -> String {
    let mut result = Vec::<String>::new();
    
    for char in input.chars() {
        if char != '!' {
            result.push(char.to_string());
        }
    }
    
    return result;
}
