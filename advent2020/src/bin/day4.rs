use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use regex::Regex;

fn parse_passports(contents: &str) -> Vec<HashMap<String, String>> {
    let mut result = Vec::new();
    // Boo Windows carriage returns :-P
    for group in contents.split("\r\n\r\n") {
        let mut passport = HashMap::new();
        for pair in group.split_whitespace() {
            let mut pair = pair.split(":");
            passport.insert(pair.next().unwrap().to_string(), pair.next().unwrap().to_string());
        }
        result.push(passport);
    }
    result
}

fn has_req_keys(passport: &HashMap<String, String>) -> bool {
    let req_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let req_keys: HashSet<String> = req_keys.iter().map(|x| x.to_string()).collect();

    let keys: HashSet<String> = passport.keys().map(|x| x.clone()).collect(); 
    req_keys.is_subset(&keys)
}

fn validate(passport: &HashMap<String, String>) -> bool {
    if !has_req_keys(passport) {
        return false
    }

    let birth_year = passport.get("byr").unwrap().parse::<i32>().unwrap();
    if birth_year < 1920 || birth_year > 2002 {
        return false
    }

    let issue_year = passport.get("iyr").unwrap().parse::<i32>().unwrap();
    if issue_year < 2010 || issue_year > 2020 {
        return false
    }

    let expire_year = passport.get("eyr").unwrap().parse::<i32>().unwrap();
    if expire_year < 2020 || expire_year > 2030 {
        return false
    }

    let height = passport.get("hgt").unwrap();
    if height.ends_with("cm") {
        let cms = height.get(..height.len() - 2).unwrap().parse::<i32>().unwrap();
        if cms < 150 || cms > 193 {
            return false
        }
    } else if height.ends_with("in") {
        let ins = height.get(..height.len() - 2).unwrap().parse::<i32>().unwrap();
        if ins < 59 || ins > 76 {
            return false
        }
    } else {
        return false
    }

    let hair_color = passport.get("hcl").unwrap();
    let hair_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    if !hair_regex.is_match(hair_color) {
        return false
    }

    let eye_color = passport.get("ecl").unwrap();
    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&eye_color[..]) {
        return false
    }

    let passport_id = passport.get("pid").unwrap();
    let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();
    if !pid_regex.is_match(passport_id) {
        return false
    }

    return true
}

fn part1(passports: &Vec<HashMap<String, String>>) -> usize {
    passports.iter().filter(|x| has_req_keys(x)).count()
}

fn part2(passports: &Vec<HashMap<String, String>>) -> usize {
    passports.iter().filter(|x| validate(x)).count()
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let passports = parse_passports(&contents);

    let result1 = part1(&passports);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&passports);
    println!("Part 2 answer: {}", result2);
}
