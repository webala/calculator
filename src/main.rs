use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap(); //chars method returns an iterator of charachters in a string
                                                        //next lets us get the next value in the iterator whech is the first char    
                                                        //we unwrap because it returns an option    
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let result = calculate(operator, first_number, second_number);

    println!("{:?}", output(first_number, second_number, operator, result));
}


// Claculate using if else statement
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 { //Rust has implicit return hence no need for return statement
    if operator == '+' {
        return first_number + second_number;
    } else if operator == '-' {
        first_number - second_number
    } else if operator == '/' {
        first_number / second_number
    } else if operator == '*' {
        first_number * second_number
    } else {
         0.0
    }
}

//We can also use Rusts match operator whish is similar to switch statement
fn calculate(operator: char, first_number: f32, second_number: f32) ->f32 {
    match operator {
        '+' => first_number + second_number, // Again, the values are implicitly returned(no return statement or semi-colons)
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number, //use bitwise or so as to use x for multiplication. Otherwise we have to escape * (\*)
        _ => panic!("Invalid operator used") //Handle other cases (default case)
    }
}

fn output(first_number: f32, second_number: f32, operator: char, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result) //No need for return statement or semi-colon(implicit return)
}