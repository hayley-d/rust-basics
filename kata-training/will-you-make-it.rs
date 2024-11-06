// Considering these factors, write a function that tells you if it is possible to get to the pump or not.
// Function should return true if it is possible and false if not.

fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    return if mpg*gallons >= distance_to_pump {true} else {false};
}

// version 2 
fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
  distance_to_pump <= mpg * gallons
}

// Improvement
fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    let distance_to_pump_km = distance_to_pump as f32 * 1.609344; // convert to reasonable units first
    let litres = gallons as f32 * 3.78541178;
    let km_per_liter = mpg as f32 * 1.609344 / 3.78541178;
    distance_to_pump_km <= km_per_liter * litres 
}
