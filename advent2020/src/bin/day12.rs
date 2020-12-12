use std::env;
use std::fs;

struct Ship {
    // Directions are 0 = east, 1 = north
    // 2 = west, 3 = south.
    // Left increments, Right decrements
    dir: usize,
    // East is positive x, West is negative x
    x: i32,
    // North is positive y, South is negative y
    y: i32,
}

#[derive(Debug)]
struct Step {
    action: char,
    magnitude: i32,
}

fn parse_steps(contents: &str) -> Vec<Step> {
    let mut result = Vec::new();

    for line in contents.lines() {
        let mut chars = line.chars();
        let action = chars.next().unwrap();
        let magnitude = chars.collect::<String>().parse::<i32>().unwrap();
        result.push(Step{ action, magnitude });
    }

    result
}

fn move_dir(ship: &mut Ship, dir: char, magnitude: i32) {
    if dir == 'N' {
        ship.y += magnitude;
    } else if dir == 'S' {
        ship.y -= magnitude;
    } else if dir == 'E' {
        ship.x += magnitude;
    } else if dir == 'W' {
        ship.x -= magnitude;
    }
}

fn move_ship(ship: &mut Ship, step: &Step) {
    if step.action == 'L' {
        let turns = (step.magnitude / 90) as usize;
        ship.dir = (ship.dir + turns) % 4;
    } else if step.action == 'R' {
        let turns = (step.magnitude / 90) as usize;
        ship.dir = (ship.dir + 3 * turns) % 4;
    } else {
        let dirs = vec!['E', 'N', 'W', 'S'];
        let mut dir = step.action;
        if step.action == 'F' {
            dir = dirs[ship.dir];
        }

        move_dir(ship, dir, step.magnitude);
    }
}

fn part1(steps: &Vec<Step>) -> i32 {
    let mut ship = Ship { dir: 0, x: 0, y: 0 };
    for step in steps {
        move_ship(&mut ship, &step);
    }
    ship.x.abs() + ship.y.abs()
}

fn rot_left(waypoint: &mut Ship) {
    let old_x = waypoint.x;
    waypoint.x = -waypoint.y;
    waypoint.y = old_x;
}

fn move_waypoint(waypoint: &mut Ship, step: &Step) {
    if step.action == 'L' {
        let turns = (step.magnitude / 90) % 4;
        for _ in 0..turns {
            rot_left(waypoint);
        }
    } else if step.action == 'R' {
        let turns = (3 * step.magnitude / 90) % 4;
        for _ in 0..turns {
            rot_left(waypoint);
        }
    } else {
        move_dir(waypoint, step.action, step.magnitude);
    }
}

fn part2(steps: &Vec<Step>) -> i32 {
    let mut ship = Ship { dir: 0, x: 0, y: 0 };
    let mut waypoint = Ship { dir: 0, x: 10, y: 1 };
    for step in steps {
        if step.action == 'F' {
            ship.x += waypoint.x * step.magnitude;
            ship.y += waypoint.y * step.magnitude;
        } else {
            move_waypoint(&mut waypoint, &step);
        }
    }
    ship.x.abs() + ship.y.abs()
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let steps = parse_steps(&contents);

    let result1 = part1(&steps);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&steps);
    println!("Part 2 answer: {}", result2);
}
