use std::env::{args, Args};


fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_num = first.parse::<f32>().unwrap();
    let second_num = second.parse::<f32>().unwrap();
    let result = operate(operator, first_num, second_num);

    println!("{:?}", output(operator, first_num, second_num, result));

}

fn operate(operator: char, first_operand: f32, second_operand: f32) -> f32 {
    match operator {
        '+' => first_operand + second_operand,
        '-' => first_operand - second_operand,
        '*' | 'x' | 'X' => first_operand * second_operand,
        '/' => first_operand / second_operand,
        _ => panic!("Invalid operation!")
    }
}

fn output(operator: char, first_operand: f32, second_operand: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_operand, operator, second_operand, result)
}