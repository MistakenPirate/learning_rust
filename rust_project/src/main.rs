fn main() {
    let x: i32 = -5;
    let y: u32 = 1000;
    let z: f32 = 1000.001;

    //if we dont give a type rust by default infers any integer as i32 and float as f64

    print!("x = {}, y = {}, z = {}\n", x, y, z);
    println!("Welcome to my first rust code!")
}
