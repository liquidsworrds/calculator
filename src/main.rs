use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first_arg = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second_arg = args.nth(0).unwrap();

    let first_num: f64 = first_arg.parse().unwrap();
    let second_num: f64 = second_arg.parse().unwrap();

    let result = operate(operator, first_num, second_num);
    println!("{:?}", output(first_num, operator, second_num, result));
}

fn operate(operator: char, first_num: f64, second_num: f64) -> f64 {
    match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '*' | 'x' => first_num * second_num,
        '/' => first_num / second_num,
         _  => panic!("Invalid operator used!")
    }
}

fn output(first_num: f64, operator: char, second_num: f64, result: f64) -> String {
    return format!("{} {} {} = {}", first_num, operator, second_num, result);
}
