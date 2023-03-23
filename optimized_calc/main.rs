use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    loop {
        println!("Please enter your first number:");

        let num1 = read_number();

        println!("Please enter your second number:");

        let num2 = read_number();

        println!("Please enter the operation you would like to perform (+, -, *, /, %, ^, !, sqrt):");

        let operation = read_operation();

        let result = match operation {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            '%' => num1 % num2,
            '^' => num1.powf(num2),
            '!' => factorial(num1),
            's' => sqrt(num1),
            _ => {
                println!("Invalid operation");
                continue;
            }
        };

        println!("The result of {} {} {} is {}", num1, operation, num2, result);

        if !continue_calculation() {
            break;
        }
    }
}

fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number")
        }
    }
}

fn read_operation() -> char {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let operation = input.trim().chars().next();

        match operation {
            Some(op) => return op,
            None => println!("Please enter a valid operation")
        }
    }
}

fn continue_calculation() -> bool {
    loop {
        println!("Would you like to perform another calculation? (y/n)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Invalid input")
        }
    }
}

fn factorial(num: f64) -> f64 {
    if num < 0.0 {
        return f64::NAN;
    } else if num == 0.0 {
        return 1.0;
    } else {
        return num * factorial(num - 1.0);
    }
}

fn sqrt(num: f64) -> f64 {
    if num < 0.0 {
        return f64::NAN;
    } else {
        return num.sqrt();
    }
}
