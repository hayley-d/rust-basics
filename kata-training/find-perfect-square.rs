/*
Complete the findNextSquare method that finds the next integral perfect square after the one passed as a parameter. Recall that an integral perfect square is an integer n such that sqrt(n) is also an integer.

If the argument is itself not a perfect square then return either -1 or an empty value like None or null, depending on your language. You may assume the argument is non-negative.
*/
fn find_next_square_v1(sq: u64) -> Option<u64> {
    if ((sq as f64).sqrt()*(sq as f64).sqrt()) as u64 == sq {
        return (((sq as f64).sqrt()) as u64) +1;    
    }
    
    -1
}

fn find_next_square(sq: u64) -> Option<u64> {
    let root = (sq as f64).sqrt();
    if root.fract() == 0.0 {
        let next_root = root as u64 + 1;
        return Some(next_root * next_root);
    }
    None
}

fn find_next_square_v2(sq: u64) -> Option<u64> {
    let root = (sq as f64).sqrt();
    if root != root.floor() {
        return None
    }
    Some((root as u64 + 1).pow(2))
}

fn find_next_square_v3(sq: u64) -> Option<u64> {
    let sq = (sq as f64).sqrt();
    if sq.fract() == 0.0 {
        Some((sq as u64 + 1u64).pow(2))
    } else {None}
}
