fn main() {

    // Tuple
    let human: (String, i32, bool) = ("Atan".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let animal_slices: &[&str] = &["Lion","Snake","Crocodiles"];
    println!("Animal Slice: {:?}", animal_slices);

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");   
    println!("Stone Cold Says: {}", stone_cold);
}

