fn printer_error(s: &str) -> String {
    let mut count : i32 = 0;
    for c in s.chars() {
        if c > 'm' {
            count += 1;
        } 
    }
    
     format!("{}/{}", count,s.len())
}

fn printer_error(s: &str) -> String {
    let count = s.chars().filter(|&c| c > 'm').count();
    format!("{}/{}", count, s.len())
}
