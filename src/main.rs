use std::io;
fn main() {
    //print message to terminal
    println!("Enter your weight in Earth");
    //create new string variable called input
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input) //reads the input from user
        .expect("Failed to read line"); //expect() handles any errors

    let weight: f32 = input.trim().parse().expect("Please type a number!"); //trim() removes whitespace, parse()converts string to f32

    let mut mars_weight = calculate_weight_on_mars(weight);

    mars_weight = mars_weight * 1000.0;

    println!("Your weight in Mars is {} gr", mars_weight);
}

fn calculate_weight_on_mars(w: f32) -> f32 {
    (w / 9.81) * 3.711
}
