/*Write a function feast that takes the animal's name and dish as arguments and returns true or false to indicate whether the beast is allowed to bring the dish to the feast.
Assume that beast and dish are always lowercase strings, and that each has at least two letters. beast and dish may contain hyphens and spaces, but these will not appear at the beginning or end of the string. They will not contain numerals.
*/
// Version 1
fn feast(beast: &str, dish: &str) -> bool {
    for word in beast.split_whitespace() {
        for part in dish.split_whitespace() {
            if word == part {
                return false;
            }
        }
    }
    
    true
}

// Version 2
fn feast_v2(beast: &str, dish: &str) -> bool {
    for word in beast.split_whitespace() {
        if dish.contains(word) {
            return false;
        }
    }   
    true
}

fn feast_v3(beast: &str, dish: &str) -> bool {
    for word in beast.split_whitespace() {
        if dish.contains(word) {
            return false;
        }
    }
    
    if &beast[..1] == &dish[..1] && &beast[beast.len()-1..] == &dish[dish.len()-1..] {
        return true;
    }
    
    false
}

fn feast_v4(beast: &str, dish: &str) -> bool {
    
    if &beast[..1] == &dish[..1] && &beast[beast.len()-1..] == &dish[dish.len()-1..] {
        return true;
    }
    
    false
}

fn feast_v5(beast: &str, dish: &str) -> bool {
    beast.chars().next() == dish.chars().next() &&
    beast.chars().last() == dish.chars().last()
}

fn feast_v6(beast: &str, dish: &str) -> bool {
    dish[..1] == beast[..1] && dish[dish.len()-1..] == beast[beast.len()-1..]
}

