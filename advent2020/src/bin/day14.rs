use std::collections::HashMap;
use std::env;
use std::fs;

use regex::Regex;

struct Write {
    index: u64,
    val: u64,
}

struct Step {
    is_mask: bool,
    mask: Option<String>,
    write: Option<Write>,
}

fn parse_input(contents: &str) -> Vec<Step> {
    let mask_regex = Regex::new(r"^mask = (.*)$").unwrap();
    let write_regex = Regex::new(r"^mem\[(.*)\] = (.*)$").unwrap();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.starts_with("mask") {
            let mask = mask_regex.captures(line).unwrap().get(1).unwrap().as_str().to_string();
            result.push(Step { is_mask: true, mask: Some(mask), write: None });
        } else {
            let captures = write_regex.captures(line).unwrap();
            let index = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let val = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let write = Write { index, val };
            result.push(Step { is_mask: false, mask: None, write: Some(write) });
        }
    }
    result
}

fn get_new_masks1(map: &String) -> (u64, u64) {
    let mut zero_map = 0;
    let mut ones_map = 0;

    for c in map.chars() {
        zero_map <<= 1;
        ones_map <<= 1;
        if c == '0' {
            zero_map |= 1;
        } else if c == '1' {
            ones_map |= 1;
        }
    }

    (!zero_map, ones_map)
}

fn part1(steps: &Vec<Step>) -> u64 {
    // AND a number with this mask to apply the zeros 
    let mut zero_mask = !0;
    // OR a number with this mask to apply the ones
    let mut ones_mask = 0;
    let mut registers = HashMap::new();
    for step in steps {
        if step.is_mask {
            let new_maps = get_new_masks1(step.mask.as_ref().unwrap());
            zero_mask = new_maps.0;
            ones_mask = new_maps.1;
        } else {
            let index = step.write.as_ref().unwrap().index;
            let mut val = step.write.as_ref().unwrap().val;
            val = (val | ones_mask) & zero_mask;

            registers.insert(index, val);
        }
    }

    registers.values().sum()
}

fn get_new_masks2(map: &String) -> (u64, u64) {
    let mut x_map = 0;
    let mut ones_map = 0;

    for c in map.chars() {
        x_map <<= 1;
        ones_map <<= 1;
        if c == 'X' {
            x_map |= 1;
        } else if c == '1' {
            ones_map |= 1;
        }
    }

    (!x_map, ones_map)
}

fn get_all_floating_masks(x_mask: u64) -> Vec<u64> {
    let mut masks = vec![0];
    let mut index = 0;
    // just so I can refer to it as mutable :-P
    let mut x_mask = x_mask;
    while x_mask > 0 {
        if x_mask & 1 == 0 {
            let new_digit = 1 << index;
            let cur_masks_len = masks.len();
            for n in 0..cur_masks_len {
                masks.push(masks[n] | new_digit);
            }
        }
        x_mask >>= 1;
        index += 1;
    }
    masks
}

fn part2(steps: &Vec<Step>) -> u64 {
    // 0s in this map correspond to areas that are floating 
    let mut x_mask = !0;
    // this will eventually hold all the floating maps we can OR with
    let mut floating_masks = vec![0];
    // OR a number with this mask to apply the ones
    let mut ones_mask = 0;
    let mut registers = HashMap::new();
    for step in steps {
        if step.is_mask {
            let new_maps = get_new_masks2(step.mask.as_ref().unwrap());
            x_mask = new_maps.0;
            ones_mask = new_maps.1;
            floating_masks = get_all_floating_masks(x_mask);
        } else {
            let mut index = step.write.as_ref().unwrap().index;
            index = (index | ones_mask) & x_mask;
            let val = step.write.as_ref().unwrap().val;
            for float in &floating_masks {
                let floating_index = index | float;
                registers.insert(floating_index, val);
            }
        }
    }

    registers.values().sum()
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
