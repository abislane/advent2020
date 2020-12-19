use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone)]
struct Rule {
    label: i32,
    expansions: Vec<Vec<i32>>,
    terminal: Option<char>,
}

struct Input {
    rules: HashMap<i32, Rule>,
    messages: Vec<String>,
}

fn parse_rule(line: &str) -> Rule {
    let mut parts = line.split(": ");
    let label = parts.next().unwrap().parse().unwrap();
    let contents = parts.next().unwrap();
    if contents.starts_with("\"") {
        let terminal = contents.chars().nth(1).unwrap(); 
        return Rule { label, expansions: Vec::new(), terminal: Some(terminal) }
    } else {
        let mut expansions = Vec::new();
        for subrule in contents.split(" | ") {
            let sublabels = subrule.split(" ").map(|x| x.parse().unwrap()).collect();
            expansions.push(sublabels);
        }
        return Rule { label, expansions, terminal: None }
    }
}

fn parse_input(contents: &str) -> Input {
    let mut lines = contents.lines();
    let mut rules = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let rule = parse_rule(line);
        rules.insert(rule.label, rule);
    }
    let messages: Vec<String> = lines.map(|x| x.to_string()).collect();
    Input { rules, messages }
}

fn validate_expansion(message: String, expansion: &Vec<i32>, rules: &HashMap<i32, Rule>) -> Vec<String> {
    let mut prefixes = vec![String::new()];
    for label in expansion {
        let mut new_prefixes = Vec::new();
        for pre in prefixes {
            let to_match = message.strip_prefix(&pre).unwrap().to_string();
            let matches = validate_rec(to_match, label.clone(), rules);
            for m in matches {
                let mut new_prefix = pre.clone();
                new_prefix.push_str(&m);
                new_prefixes.push(new_prefix);
            }
        }
        prefixes = new_prefixes;
    }
    prefixes
}

fn validate_rec(message: String, label: i32, rules: &HashMap<i32, Rule>) -> Vec<String> {
    let rule = rules.get(&label).unwrap();
    if rule.terminal.is_some() {
        if message.starts_with(&rule.terminal.unwrap().to_string()) {
            return vec![rule.terminal.unwrap().to_string()]
        } else {
            return vec![]
        }
    } else {
        let mut result = Vec::new();
        for expansion in &rule.expansions {
            result.append(&mut validate_expansion(message.clone(), &expansion, rules));
        }
        return result
    }
}

fn validate(message: String, rules: &HashMap<i32, Rule>) -> bool {
    validate_rec(message.clone(), 0, rules).contains(&message)
}

fn part1(input: &Input) -> usize {
    input.messages.iter().filter(|x| validate(x.to_string(), &input.rules)).count()
}

fn part2(input: &Input) -> usize {
    let mut rules = input.rules.clone();
    let rule_eight = Rule {
        label: 8,
        expansions: vec![vec![42], vec![42, 8]],
        terminal: None
    };
    rules.insert(8, rule_eight);
    let rule_eleven = Rule {
        label: 11,
        expansions: vec![vec![42, 31], vec![42, 11, 31]],
        terminal: None
    };
    rules.insert(11, rule_eleven);
    input.messages.iter().filter(|x| validate(x.to_string(), &rules)).count()
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
