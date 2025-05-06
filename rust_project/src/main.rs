fn main() {
    // let x: i32 = -5;
    // let y: u32 = 1000;
    // let z: f32 = 1000.001;
    // print!("x = {}, y = {}, z = {}\n", x, y, z);
    //if we dont give a type rust by default infers any integer as i32 and float as f64

    // let mut x: i8 = 10000;
    // for i in 0..1000 {
    //     x = x + 100;
    // }
    // print!("x = {}\n", x);
    //by default all variables in rust are mutable, unless mut is defined explicitly

    // bools
    // let is_male : bool = true;
    // let is_above_18 : bool = true;
    // if is_male{
    //     println!("You are male")
    // } else {
    //     println!("You are not a male")
    // }
    // if is_male && is_above_18{
    //     println!("You are an adult")
    // }

    let greeting : String = String::from("Hello World");
    println!("{}", greeting);
    let char1: Option<char> = greeting.chars().nth(1);
    match char1{
        Some(c) => println!("{}", c),
        None => println!("No character at index 1000"),
    }


}
