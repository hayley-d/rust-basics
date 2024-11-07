/*Write a function to convert a name into initials. This kata strictly takes two words with one space in between them.
The output should be two capital letters with a dot separating them.*/

fn abbrev_name(name: &str) -> String {
   let mut counter: usize = 0;

    for char in name.chars() {
        if char == ' ' {
            return format!("{}.{}", &name[..1], &name[counter + 1..counter + 2]);
        }
        counter += 1;
    }
    
    String::new()
}

//Version 2
fn abbrev_name_v2(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().collect();
    if parts.len() >= 2 {
        format!("{}.{}", &parts[0][..1], &parts[1][..1])
    } else {
        String::new();
    }
}

// Version 3
fn abbrev_name_v3(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .map(|c| c.to_uppercase().to_string())
        .collect::<Vec<_>>()
        .join(".")
}


