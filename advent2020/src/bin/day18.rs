use std::collections::HashMap;
use std::env;
use std::fs;

fn parse_line(contents: &str) -> Vec<String> {
    contents.replace("(", "( ").replace(")", " )").split(" ").map(|x| x.to_string()).collect()
}

fn parse_input(contents: &str) -> Vec<Vec<String>> {
    contents.lines().map(parse_line).collect()
}

fn get_postfix(tokens: &Vec<String>, prec_table: &HashMap<String, i32>) -> Vec<String> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    for token in tokens {
        if token == "(" {
            stack.push(token);
        } else if token == ")" {
            let mut top_token = stack.pop().unwrap();
            while top_token != "(" {
                result.push(top_token.clone());
                top_token = stack.pop().unwrap();
            }
        } else if token == "+" || token == "*" {
            while stack.len() > 0 && 
                (prec_table.get(stack[stack.len() - 1]).unwrap() >= prec_table.get(token).unwrap()) {
                result.push(stack.pop().unwrap().clone());
            }
            stack.push(token);
        } else {
            result.push(token.clone());
        }
    }

    while stack.len() > 0 {
        result.push(stack.pop().unwrap().clone());
    }

    result
}

fn eval_postfix(token: &Vec<String>) -> i64 {
    let mut stack = Vec::new();
    for token in token {
        if token == "+" {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a + b);
        } else if token == "*" {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a * b);
        } else {
            stack.push(token.parse::<i64>().unwrap());
        }
    }

    stack.pop().unwrap()
}

fn eval(tokens: &Vec<String>, prec_table: &HashMap<String, i32>) -> i64 {
    let postfix = get_postfix(tokens, prec_table);
    eval_postfix(&postfix)
}

fn sum_inputs(input: &Vec<Vec<String>>, prec_table: &HashMap<String, i32>) -> i64 {
    input.iter().map(|x| eval(x, prec_table)).sum()
}

fn part1(input: &Vec<Vec<String>>) -> i64 {
    let mut prec_table = HashMap::new();
    prec_table.insert(String::from("("), 0);
    prec_table.insert(String::from("+"), 1);
    prec_table.insert(String::from("*"), 1);
    sum_inputs(input, &prec_table)
}

fn part2(input: &Vec<Vec<String>>) -> i64 {
    let mut prec_table = HashMap::new();
    prec_table.insert(String::from("("), 0);
    prec_table.insert(String::from("+"), 2);
    prec_table.insert(String::from("*"), 1);
    sum_inputs(input, &prec_table)
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let input = parse_input(&contents);

    let result1 = part1(&input);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&input);
    println!("Part 2 answer: {}", result2);
}
