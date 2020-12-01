use std::env;
use std::fs;

fn parse_nums(content: &str) -> Vec<i32> {
    content.lines().map(|x| -> i32 { x.parse::<i32>().unwrap() }).collect()
}

fn part1(nums: &Vec<i32>) -> i32 {
    for index1 in 0..nums.len() {
        for index2 in 0..index1 {
            if nums[index1] + nums[index2] == 2020 {
                return nums[index1] * nums[index2]
            }
        }
    }
    // Should not happen with a valid input
    return -1
}

fn part2(nums: &Vec<i32>) -> i32 {
    for index1 in 0..nums.len() {
        for index2 in 0..index1 {
            for index3 in 0..index2 {
                if nums[index1] + nums[index2] + nums[index3] == 2020 {
                    return nums[index1] * nums[index2] * nums[index3]
                }
            }
            
        }
    }
    // Should not happen with a valid input
    return -1
}

fn main() {
    let mut args = env::args();
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let nums = parse_nums(&contents);

    let result1 = part1(&nums);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&nums);
    println!("Part 2 answer: {}", result2);
}
