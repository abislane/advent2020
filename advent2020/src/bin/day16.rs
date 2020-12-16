use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use regex::Regex;

struct Field<'a> {
    name: &'a str,
    intv1: (i32, i32),
    intv2: (i32, i32),
}

struct Input<'a> {
    fields: Vec<Field<'a>>,
    your_ticket: Vec<i32>,
    other_tickets: Vec<Vec<i32>>,
}

fn parse_input(contents: &str) -> Input {
    let mut lines = contents.lines();
    let mut fields = Vec::new();
    let field_regex = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        } 
        let captures = field_regex.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let low1 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let high1 = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let low2 = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let high2 = captures.get(5).unwrap().as_str().parse::<i32>().unwrap();
        fields.push(Field { name: name, intv1: (low1, high1), intv2: (low2, high2)});
    }

    // Skips the "Your Ticket" Line
    lines.next();
    let your_ticket: Vec<i32> = lines.next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    // Skip blank line
    lines.next();
    // Skip the "Nearby tickets" line
    lines.next();
    
    let mut other_tickets = Vec::new();
    for line in lines {
        other_tickets.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
    }

    Input { fields, your_ticket, other_tickets }
}

fn val_in_field(val: i32, field: &Field) -> bool {
    (val >= field.intv1.0 && val <= field.intv1.1) || (val >= field.intv2.0 && val <= field.intv2.1)
}

fn val_in_fields(val: i32, fields: &Vec<Field>) -> bool {
    for field in fields {
        if val_in_field(val, field) {
            return true
        }
    }
    return false
}

fn part1(input: &Input) -> i32 {
    let mut result = 0;
    for other in &input.other_tickets {
        for &val in other {
            if !val_in_fields(val, &input.fields) {
                result += val;
            }
        }
    }
    result
}

fn valid_tickets(input: &Input) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for other in &input.other_tickets {
        let mut valid = true;
        for &val in other {
            if !val_in_fields(val, &input.fields) {
                valid = false;
                break;
            }
        }
        if valid {
            result.push(other.clone());
        }
    }
    result
}

fn get_possible_indexes(field: &Field, tickets: &Vec<Vec<i32>>) -> HashSet<usize> {
    let size = tickets[0].len();
    let mut valid = Vec::new();
    for _ in 0..size {
        valid.push(true);
    }

    for ticket in tickets {
        for i in 0..size {
            if !(val_in_field(ticket[i], field)) {
                valid[i] = false;
            }
        }
    }

    let mut result = HashSet::new();
    for i in 0..size {
        if valid[i] {
            result.insert(i);
        }
    }
    result
}

fn get_single_keys<'a>(
    possible_indexes: &HashMap<&'a str, HashSet<usize>>,
    used_vals: &HashSet<usize>
) -> Vec<(&'a str, usize)> {
    let mut result = Vec::new();

    for (&key, val) in possible_indexes {
        let mut unused_vals = val.clone();
        for used in used_vals {
            unused_vals.remove(used);
        }
        if unused_vals.len() == 1 {
            let val = unused_vals.iter().next().unwrap().clone();
            result.push((key, val));
        }
    }

    result
}

fn part2(input: &Input) -> i64 {
    let valid_tickets = valid_tickets(input);
    let mut possible_indexes = HashMap::new();
    let mut used_indexes = HashSet::new();
    for field in &input.fields {
        possible_indexes.insert(field.name, get_possible_indexes(&field, &valid_tickets));
        println!("{} has possible indexes {:?}", field.name, possible_indexes.get(field.name).unwrap());
    }

    let mut result = 1;
    while possible_indexes.len() > 0 {
        let single_keys = get_single_keys(&possible_indexes, &used_indexes);
        for (key, val) in single_keys {
            println!("{} has a value of {} on your ticket", key, input.your_ticket[val]);
            if key.starts_with("departure") {
                result *= input.your_ticket[val] as i64;
            }
            used_indexes.insert(val);
            possible_indexes.remove(key);
        }
    }

    result
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
