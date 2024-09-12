pub enum Shape {
    Rect(f64, f64), // Renamed 'rect' to 'Rect' to follow Rust naming conventions
}

pub fn enum_check() {
    fn area() -> f64 {
        let rect = Shape::Rect(1.0, 2.0);
        calculate_area(rect)
    }

    fn calculate_area(shape: Shape) -> f64 {
        match shape {
            Shape::Rect(a, b) => a * b,
        }
    }

    // Call the area function and print the result
    let result = area();
    println!("The area of the rectangle is: {}", result);
}
