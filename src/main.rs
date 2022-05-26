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

    let result = operate(operator, first_number, second_number);

    println!("{:?}", output(first_number, second_number, operator, result));
}


fn operate(operator: char, first_number: f32, second_number: f32) -> f32{ //Rust has implicit return hence no need for return statement
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

fn output(first_number: f32, second_number: f32, operator: char, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result) //No need for return statement or semi-colon(implicit return)
}