use simple_exercises::{*};
use std::io::stdin;
fn main() {
    println!("reverse_integer - {} - {}", 125, reverse_integer(125));
    println!("divide - {} - {} -  {}", 22, 4, divide(22,4));
    compute_circle_area();
}

fn compute_circle_area(){
    println!("Please Input the radius of the Circle: ");

    let mut radius = String::new();
    stdin().read_line(&mut radius).expect("Failed to Read the User Input");

    let radius : u32 = radius.trim().parse().expect("Failed to parse the input, please Input a number");

    let area = get_circle_area(radius as f64);

    println!("Area of Circle with radius {} = {}", radius, area);
}