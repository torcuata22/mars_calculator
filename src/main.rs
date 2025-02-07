fn main() {
    let weight = calculate_weight_on_mars(100.0);
    println!("Your weight in Mars is {} Kg", weight);
}

fn calculate_weight_on_mars(w: f32) -> f32 {
    (w / 9.81) * 3.711
}
