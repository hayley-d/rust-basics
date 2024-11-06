// Considering these factors, write a function that tells you if it is possible to get to the pump or not.
// Function should return true if it is possible and false if not.

fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    return if mpg*gallons >= distance_to_pump {true} else {false};
}

// version 2 
fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
  distance_to_pump <= mpg * gallons
}
