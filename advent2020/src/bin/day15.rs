use std::collections::HashMap;
use std::env;
use std::fs;

fn parse_input(contents: &str) -> Vec<usize> {
    contents.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect() 
}

fn do_game(input: &Vec<usize>, num_turns: usize) -> usize {
    let mut last_index = HashMap::with_capacity(num_turns);
    for index in 0..(input.len() - 1) {
        last_index.insert(input[index], index + 1);
    }

    let mut size = input.len();
    let mut cur_num = input[size - 1];

    while size < num_turns {
        let prev_index = last_index.get(&cur_num).unwrap_or(&0).clone();
        let next_num = if prev_index == 0 {
            0
        } else {
            size - prev_index
        };
        last_index.insert(cur_num, size);
        cur_num = next_num;
        size += 1;
    }

    cur_num
}

fn part1(input: &Vec<usize>) -> usize {
    do_game(input, 2020)
}

fn part2(input: &Vec<usize>) -> usize {
    do_game(input, 30000000)
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
