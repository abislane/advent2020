use std::env;
use std::fs;

struct Schedule<'a> {
    earliest: i128,
    buses: Vec<&'a str>,
}

fn parse_schedule(contents: &str) -> Schedule {
    let mut lines = contents.lines();
    let earliest = lines.next().unwrap().parse::<i128>().unwrap();
    let buses : Vec<&str> = lines.next().unwrap().split(",").collect();
    Schedule { earliest, buses }
}

fn part1(schedule: &Schedule) -> i128 {
    let mut best = i128::MAX;
    let mut best_route = -1;

    for &bus in &schedule.buses {
        if bus == "x" {
            continue;
        }

        let route_number = bus.parse::<i128>().unwrap();
        let mut bus_num = schedule.earliest / route_number;
        if schedule.earliest % route_number != 0 {
            bus_num += 1;
        }
        let bus_time = route_number * bus_num;
        if bus_time < best {
            best = bus_time;
            best_route = route_number;
        }
    }

    (best - schedule.earliest) * best_route
}

// returns (x, y) such that x*a + y*b = gcd(a, b)
fn bezout_numbers(a: i128, b: i128) -> (i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        
        let temp = r;
        r = old_r - q * r;
        old_r = temp;

        let temp = s;
        s = old_s - q * s;
        old_s = temp;

        let temp = t;
        t = old_t - q * t;
        old_t = temp;
    }
    (old_s, old_t)
}

// find the solution for x = a1 mod m1, x = a2 mod m2
// where m1 and m2 are coprime. Return the solution between 0 and m1 * m2
fn chinese_remainder_theorem(a1: i128, m1: i128, a2: i128, m2: i128) -> i128 {
    let (n1, n2) = bezout_numbers(m1, m2);
    let mut result = a1 * m2 * n2 + a2 * m1 * n1;
    let m = m1 * m2;
    result = result % m;
    while result < 0 {
        result += m;
    }
    while result >= m {
        result -= m;
    }
    result
}

fn part2(schedule: &Schedule) -> i128 {
    let mut result = 0;
    let mut m = 1;
    for index in 0..schedule.buses.len() {
        if schedule.buses[index] == "x" {
            continue;
        }
        let route_number = schedule.buses[index].parse::<i128>().unwrap();
        let mut a = route_number - (index as i128);
        while a < 0 {
            a += route_number;
        }
        result = chinese_remainder_theorem(result, m, a, route_number);
        m *= route_number;
    }
    result
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let schedule = parse_schedule(&contents);

    let result1 = part1(&schedule);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&schedule);
    println!("Part 2 answer: {}", result2);
}
