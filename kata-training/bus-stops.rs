/*

*/

// Version 1
fn number(bus_stops:&[(i32,i32)]) -> i32 {
    let mut total : i32 = 0;
    for tup in bus_stops.iter() {
        total += tup.0; 
        total -= tup.1; 
    }
    
    total
}

// Version 2
fn number_v2(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().map(|(on, off)| on - off).sum()
}
