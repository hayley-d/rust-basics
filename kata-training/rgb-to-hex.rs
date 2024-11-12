/*
The rgb function is incomplete. Complete it so that passing in RGB decimal values will result in a hexadecimal representation being returned. 
Valid decimal values for RGB are 0 - 255. Any values that fall out of that range must be rounded to the closest valid value.
*/
fn rgb(r: i32, g: i32, b: i32) -> String {
    fn clamp(value: i32) -> i32 {
        value.max(0).min(255)
    }
    format!("{:02X}{:02X}{:02X}", clamp(r), clamp(g), clamp(b))
}
