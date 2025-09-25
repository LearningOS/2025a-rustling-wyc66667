use std::f32::consts::PI;

fn main() {
    let radius = 5.0f32;
    let area = PI * radius.powi(2);
    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}