//given a number and have to make it negative. But maybe the number is already negative?
fn make_negative(n: i32) -> i32 {
    if n > 0 {
        return n*-1;
    }
    return n;
}

//version 2
fn make_negative_2(n: i32) -> i32 {
    -n.abs()
}

//version 3
fn make_negative_3(n: i32) -> i32 {
    match n {
        1.. => -n,
        ..=0 => n
    }
}

// version 4
fn make_negative_4(n: i32) -> i32 {
    if n.is_positive() {-n} else {n}
}


