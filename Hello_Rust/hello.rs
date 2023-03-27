fn add (num1: i32, num2: i32)-> i32 {
    num1 + num2
}

fn subtract (num1: f32, num2: f32)-> f32{
    return num1 - num2
}

fn main () {
    println!("{} {}", add(23.3 as i32, 123), subtract(23.345, 45 as f32))
}