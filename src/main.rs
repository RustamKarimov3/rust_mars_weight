use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    println!("Weight on mars: {}", calculate_mars_weight(weight));
}

fn calculate_mars_weight(weight: f32) -> f32 {
    weight/9.8 * 3.7
}