use simple_exercises::{*};
use std::io::{stdin, stdout, Write};
fn main() {

    println!("List of programs you can run. ");
    println!("1. Circle Area Calculator");
    println!("2. Quadratic Equation Solver");

    print!("Input the number of the program you want to run : ");
    stdout().flush().unwrap();
    let selected_num : u32 = get_input().trim().parse().expect("Failed tp parse input, please input one of the listed numbers");

    match selected_num {
        1 => compute_circle_area(),
        2 => solve_quadratic_equation(),
        _ => println!("Inavlid input, Please input a number in the list!")
    }
}

fn compute_circle_area(){
    print!("Please Input the radius of the Circle: ");
    stdout().flush().unwrap();

    let radius : u32 = get_input().trim().parse().expect("Failed to parse the input, please Input a number");

    let area = get_circle_area(radius as f64);

    println!("Area of Circle with radius {} = {}", radius, area);
}

fn solve_quadratic_equation(){
    println!("Let's solve quadratic equations!  ax² + bx + c");

    print!("Input a = ");
    stdout().flush().unwrap();

    let a : f64 = get_input().trim().parse().expect("Failed to parse the input, please Input a number");

    print!("Input b = ");
    stdout().flush().unwrap();

    let b : f64 = get_input().trim().parse().expect("Failed to parse the input, please Input a number");

    print!("Input c = ");
    stdout().flush().unwrap();

    let c : f64 = get_input().trim().parse().expect("Failed to parse the input, please Input a number");
    
    let equation = format!("{}x² {} {}x {} {}", a, if b < 0.0 { '-' } else { '+' }, b.abs(), if c < 0.0 { '-'} else {'+'} ,c.abs());

    if a == 0.0 {
        println!("{} is not a quadratic equation", equation);
    } else {
        match get_quadratic_equation_roots(a, b, c) {
            Some((first_root, second_root)) => println!("The roots of the quadratic equation {} are {} and {}", 
                equation, first_root, second_root),
            None => println!("The quadratic equation {} has no roots", equation)
        }
    }
    
}

fn get_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read input");
    buffer
}
