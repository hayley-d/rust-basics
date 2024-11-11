/*
accum("abcd") -> "A-Bb-Ccc-Dddd"
accum("RqaEzty") -> "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy"
accum("cwAt") -> "C-Ww-Aaa-Tttt"
*/
fn accum(s:&str)->String {
    s.chars().enumerate().map(|(i,c)| {
        let mut result = c.to_uppercase().to_string();
        result.push_str(&c.to_lowercase().to_string().repeat(i));
        result
    })
    .collect::<Vec<String>>()
    .join("-")
}
