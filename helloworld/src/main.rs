use std::io;
fn main() {

    hello_world();
    human_id("Atan", 40, 170.0);
    let value: i32 = add(4, 6);

    println!("Add value is: {}", value)
}

fn hello_world() {
    println!("Hello, Rust!");
}

fn human_id(name: &str, age: u32, height: f32){
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {} cm", height);
}

// function returning values
fn add (a: i32, b: i32) -> i32 {
    a + b
    // anything without semicolon (;) is a returning value
}

// fn BMI () {

// }


