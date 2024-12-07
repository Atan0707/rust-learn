use std::io;
fn main() {

    // hello_world();
    // human_id("Atan", 40, 170.0);
    // let value: i32 = add(4, 6);

    // println!("Add value is: {}", value)


    let mut height_input = String::new();
    println!("Enter your height in cm: ");

    // Read input
    io::stdin()
        .read_line(&mut height_input)
        .expect("Failed to read line");

    let height: f32 = height_input.trim().parse().expect("Please enter a valid number");


    println!("Enter your weight: ");
    let mut weight_input = String::new();
    io::stdin()
        .read_line(&mut weight_input)
        .expect("Failed to read line");

    let weight: f32 = weight_input.trim().parse().expect("Please enter a valid number");



    let result = bmi(height, weight);
    println!("Your BMI is: {}", result);
    
    check_bmi(result);

}

// fn hello_world() {
//     println!("Hello, Rust!");
// }

// fn human_id(name: &str, age: u32, height: f32){
//     println!("Name: {}", name);
//     println!("Age: {}", age);
//     println!("Height: {} cm", height);
// }

// // function returning values
// fn add (a: i32, b: i32) -> i32 {
//     a + b
//     // anything without semicolon (;) is a returning value
// }

fn bmi (height: f32, weight: f32) -> f32{
    let height_squared: f32 = (height / 100.0).powi(2);
    let calculate_bmi: f32 = weight / height_squared;
    return calculate_bmi;
}

fn check_bmi (bmi: f32) {
    if bmi < 18.5 {
        println!("You're underweight");
    } else if bmi >= 18.5 && bmi < 30.0 {
        println!("You're normal");
    } else if bmi >= 25.0 && bmi < 40.0 {
        println!("You're overweight");
    } else {
        println!("You're obese");
    }
}