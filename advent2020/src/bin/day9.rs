use std::collections::HashSet;
use std::cmp;
use std::env;
use std::fs;

fn parse_message(contents: &str) -> Vec<i64> {
    contents.lines().map(|x| x.parse::<i64>().unwrap()).collect() 
}

fn get_pairs(message: &Vec<i64>, start_index: usize, preamble_length: usize) -> HashSet<i64> {
    let mut result = HashSet::new();

    for i in start_index..(start_index + preamble_length) {
        for j in start_index..i {
            result.insert(message[i] + message[j]);
        }
    }

    result
}

fn part1(message: &Vec<i64>, preamble_length: usize) -> i64 {
    for index in 0..(message.len() - preamble_length) {
        let sums = get_pairs(message, index, preamble_length);
        let target = message[index + preamble_length];
        if !sums.contains(&target) {
            return target
        }
    }
    // should not happen
    return -1
}

fn min_max_sum(seq: &[i64]) -> i64 {
    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for &x in seq {
        min = cmp::min(min, x);
        max = cmp::max(max, x);
    }

    return min + max;
}

fn part2(message: &Vec<i64>, target: i64) -> i64 {
    let mut partial_sums = Vec::new();
    partial_sums.push(message[0]);
    for index in 1..message.len() {
        partial_sums.push(partial_sums[index - 1] + message[index]);
    }

    for i in 0..partial_sums.len() {
        for j in 0..i {
            if partial_sums[i] - partial_sums[j] == target {
                return min_max_sum(&message[j..i+1]);
            }
        }
    }

    return -1
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let message = parse_message(&contents);

    let result1 = part1(&message, 25);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&message, result1);
    println!("Part 2 answer: {}", result2);
}
