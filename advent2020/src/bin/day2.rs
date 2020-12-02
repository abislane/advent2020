use std::env;
use std::fs;

struct Policy<'a> {
    low: usize,
    high: usize,
    letter: char,
    password: &'a str
}

type Validator = fn(&Policy) -> bool;

fn parse_policies(contents: &str) -> Vec<Policy> {
    let mut result = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let bounds: Vec<&str> = parts[0].split("-").collect();
        let low = bounds[0].parse::<usize>().unwrap();
        let high = bounds[1].parse::<usize>().unwrap();
        let letter = parts[1].chars().nth(0).unwrap();
        let password = parts[2];

        result.push(Policy {low, high, letter, password});
    }

    result
}

fn valid1(policy: &Policy) -> bool {
    let mut count = 0;
    for c in policy.password.chars() {
        if c == policy.letter {
            count += 1;
        }
    }

    count >= policy.low && count <= policy.high
}

fn valid2(policy: &Policy) -> bool {
    let check1 = policy.password.chars().nth(policy.low - 1).unwrap() == policy.letter;
    let check2 = policy.password.chars().nth(policy.high - 1).unwrap() == policy.letter;

    check1 ^ check2
}

fn count_valid_policies(policies: &Vec<Policy>, validator: Validator) -> i32 {
    let mut result = 0;
    for policy in policies {
        if validator(policy) {
            result += 1;
        }
    }
    result
}

fn part1(policies: &Vec<Policy>) -> i32 {
    count_valid_policies(policies, valid1)
}

fn part2(policies: &Vec<Policy>) -> i32 {
    count_valid_policies(policies, valid2)
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let policies = parse_policies(&contents);

    let result1 = part1(&policies);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&policies);
    println!("Part 2 answer: {}", result2);
}
