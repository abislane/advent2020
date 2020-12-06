use std::collections::HashSet;
use std::env;
use std::fs;

fn parse_groups(contents: &str) -> Vec<Vec<&str>> {
    let mut result = Vec::new();
    // Boo Windows carriage returns :-P
    for group in contents.split("\r\n\r\n") {
        result.push(group.lines().collect());
    }
    result
}

fn count_intersection_yeses(group: &Vec<&str>) -> usize {
    let mut result: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for form in group {
        let form: Vec<char> = form.chars().collect();
        result.retain(|x| form.contains(x));
    }
    result.len()
}

fn count_union_yeses(group: &Vec<&str>) -> usize {
    let mut result = HashSet::new();
    for form in group {
        form.chars().for_each(|x| {result.insert(x);});
    }
    result.len()
}

fn part1(groups: &Vec<Vec<&str>>) -> usize {
    groups.iter().map(|x| count_union_yeses(x)).sum()
}

fn part2(groups: &Vec<Vec<&str>>) -> usize {
    groups.iter().map(|x| count_intersection_yeses(x)).sum()
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let groups = parse_groups(&contents);

    let result1 = part1(&groups);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&groups);
    println!("Part 2 answer: {}", result2);
}
