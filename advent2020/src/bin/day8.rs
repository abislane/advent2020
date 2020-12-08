use std::env;
use std::fs;

#[derive(Clone)]
struct Instruction<'a> {
    command: &'a str,
    value: i32,
}

struct State {
    index: usize,
    acc: i32,
}

fn parse_instructions(contents: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split(" ");
        let command = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();
        result.push(Instruction{ command, value });
    }
    result
}

fn advance_state(instruction: &Instruction, state: &mut State) {
    if instruction.command == "acc" {
        state.acc += instruction.value;
        state.index += 1;
    } else if instruction.command == "jmp" {
        if (state.index as i32 + instruction.value) < 0 {
            // Because of type constraints, we can't easily show a negative
            // index. But an index of 0 will cause a loop, which captures the
            // invalid state fine
            state.index = 0;
        } else {
            state.index = (state.index as i32 + instruction.value) as usize;
        }
    } else if instruction.command == "nop" {
        state.index += 1;
    }
}

fn run_program(instructions: &Vec<Instruction>) -> State {
    // run the program until either a loop is detected, or it terminates
    // if it finds a loop, it will return the state right before executing an
    // instruction for the second time
    let mut state = State { index: 0, acc: 0 };
    let mut visited = vec![false; instructions.len()];
    while state.index < instructions.len() && !visited[state.index] {
        visited[state.index] = true;
        advance_state(&instructions[state.index], &mut state);
    }
    state
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    run_program(&instructions).acc
}

fn part2(instructions: &Vec<Instruction>) -> i32 {
    for index in 0..instructions.len() {
        if instructions[index].command == "acc" {
            continue;
        }
        let mut new_insts = instructions.clone();
        if instructions[index].command == "jmp" {
            new_insts[index] = Instruction {
                command: "nop",
                value: instructions[index].value
            };
        } else { // command == "nop"
            new_insts[index] = Instruction {
                command: "jmp",
                value: instructions[index].value
            };
        }
        let end_state = run_program(&new_insts);
        if end_state.index == instructions.len() {
            return end_state.acc;
        }
    }
    // Should not happen
    return -1;
}


fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let instructions = parse_instructions(&contents);

    let result1 = part1(&instructions);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&instructions);
    println!("Part 2 answer: {}", result2);
}
