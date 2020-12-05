use std::env;
use std::fs;

fn get_row(row_seq: &str) -> i32 {
    let mut min = 0; // note: inclusive
    let mut max = 128; // note: exclusive

    for c in row_seq.chars() {
        let mid = (min + max) / 2;
        if c == 'F' {
            max = mid;
        } else {
            min = mid;
        }
    }

    min
}

fn get_seat(seat_seq: &str) -> i32 {
    let mut min = 0; // note: inclusive
    let mut max = 8; // note: exclusive

    for c in seat_seq.chars() {
        let mid = (min + max) / 2;
        if c == 'L' {
            max = mid;
        } else {
            min = mid;
        }
    }

    min
}

fn get_seat_id(ticket: &str) -> i32 {
    let row = get_row(ticket.get(0..7).unwrap());
    let seat = get_seat(ticket.get(7..10).unwrap());

    row * 8 + seat
}

fn part1(tickets: &Vec<&str>) -> i32 {
    tickets.into_iter().map(|x| get_seat_id(x)).max().unwrap()
}

fn part2(tickets: &Vec<&str>) -> i32 {
    let mut ids: Vec<i32> = tickets.into_iter().map(|x| get_seat_id(x)).collect();
    ids.sort();

    for i in 0..ids.len()-1 {
        if ids[i + 1] - ids[i] == 2 {
            // open space, this is our place
            return ids[i] + 1
        }
    }
    // Should not reach this
    return -1
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let tickets: Vec<&str> = contents.lines().collect();

    let result1 = part1(&tickets);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&tickets);
    println!("Part 2 answer: {}", result2);
}
