#[allow(dead_code)]
struct Circle {
    radius: i32,
    unit: String
}

fn main() {
    let circle = Circle {
        radius: 4,
        unit: "cm".to_string()
    };
    println!("{}", circle.radius);
    // println!("{}", circle.unit);
}