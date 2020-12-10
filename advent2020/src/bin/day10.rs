use std::env;
use std::fs;

fn parse_adapters(contents: &str) -> Vec<i32> {
    let mut result: Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    result.sort();
    result
}

fn part1(adapters: &Vec<i32>) -> i32 {
    let mut one_changes = 0;
    let mut three_changes = 0;

    let mut prev = 0;
    for &x in adapters {
        let diff = x - prev;
        if diff == 1 {
            one_changes += 1;
        }
        if diff == 3 {
            three_changes += 1;
        }
        prev = x;
    }
    // final change to implicit adapter is always by three
    three_changes += 1;
    one_changes * three_changes
}

fn part2(adapters: &Vec<i32>) -> i64 {
    let mut all_nums = adapters.clone();
    let max = adapters[adapters.len() - 1] + 3;
    all_nums.push(max);

    let mut ways = vec![0; (max + 1) as usize];
    ways[0] = 1;
    for x in all_nums {
        // can go back by 1 2 or 3
        for index in 1..4 {
            if index > x {
                break;
            }
            ways[x as usize] += ways[(x - index) as usize];
        }
    }

    ways[max as usize]
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let adapters = parse_adapters(&contents);

    let result1 = part1(&adapters);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&adapters);
    println!("Part 2 answer: {}", result2);
}
