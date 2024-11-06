/* Create a function which translates a given DNA string into RNA.

For example:

"GCAT"  =>  "GCAU"
The input string can be of arbitrary length - in particular, it may be empty. All input is guaranteed to be valid, i.e. each input string will only ever consist of 'G', 'C', 'A' and/or 'T'.

*/
// Version 1
fn dna_to_rna(dna: &str) -> String {
    return str::replace(dna,"T","U");
}

// Version 2
fn dna_to_rna(dna: &str) -> String {
    return dna.replace("T","U");
}

// Version 3 
fn dna_to_rna(dna: &str) -> String {
     dna.chars().map(|c| match c {
        'G' => 'G',
        'C' => 'C',
        'A' => 'A',
        'T' => 'U',
        _ => panic!("Invalid DNA character"),
    }).collect()
}

// Version 4
fn dna_to_rna(dna: &str) -> String {
    let mut res = String::new();
    for s in dna.chars() {
        match s {
            'T' => res.push('U'),
            _ => res.push(s),
        }
    }
    res
}
