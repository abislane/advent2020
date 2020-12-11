use std::env;
use std::fs;

/// Parses the grid in the input, including an extra border to make looking at
/// adjacent squares easier on the edges
fn parse_grid(contents: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let lines: Vec<&str> = contents.lines().collect();
    let width = lines[0].len() + 2;
    result.push(vec!['.'; width]);
    for line in lines {
        let mut line_chars = Vec::new();
        line_chars.push('.');
        for c in line.chars() {
            line_chars.push(c);
        }
        line_chars.push('.');

        result.push(line_chars);
    }
    result.push(vec!['.'; width]);
    result
}

fn count_neighbors1(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut result = 0;
    // iterate over row - 1, row, row + 1
    for d_row in (row - 1)..(row + 2) {
        // iterate over col - 1, col, col + 1
        for d_col in (col - 1)..(col + 2) {
            if !(d_row == row && d_col == col) && grid[d_row][d_col] == '#' {
                result += 1;
            }
        }
    }

    result
} 

fn update_grid1(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for row in 0..grid.len() {
        let mut row_chars = Vec::new();
        for col in 0..grid[0].len() {
            if grid[row][col] == '.' {
                row_chars.push('.');
                continue;
            }
            let count_occupied = count_neighbors1(grid, row, col);
            if grid[row][col] == 'L' {
                if count_occupied == 0 {
                    row_chars.push('#');
                } else {
                    row_chars.push('L');
                }
            } else if grid[row][col] == '#' {
                if count_occupied >= 4 {
                    row_chars.push('L');
                } else {
                    row_chars.push('#');
                }
            }
        }
        result.push(row_chars);
    }

    result
}

fn count_occupied(grid: &Vec<Vec<char>>) -> i32 {
    let mut result = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '#' {
                result += 1;
            }
        }
    }

    result
}

fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let mut current_grid = grid.clone();
    loop {
        let new_grid = update_grid1(&current_grid);
        // print_grid(&new_grid);
        if new_grid == current_grid {
            return count_occupied(&current_grid)
        }
        current_grid = new_grid;
    }
}

fn in_bounds(grid: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    row >= 0 && col >= 0 && (row as usize) < grid.len() && (col as usize) < grid[0].len()
}

fn see_occupied_chair(grid: &Vec<Vec<char>>, row: usize, col: usize, d_row: i32, d_col: i32) -> bool {
    let mut cur_row = row as i32 + d_row;
    let mut cur_col = col as i32 + d_col;
    while in_bounds(grid, cur_row, cur_col) {
        let space = grid[cur_row as usize][cur_col as usize];
        if space == '#' {
            return true
        } else if space == 'L' {
            return false
        } else {
            cur_row += d_row;
            cur_col += d_col;
        }
    } 
    return false
}

fn count_neighbors2(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut result = 0;
    // iterate over -1, 0, 1
    for d_row in -1..2 {
        for d_col in -1..2 {
            if d_row == 0 && d_col == 0 {
                continue;
            }
            if see_occupied_chair(grid, row, col, d_row, d_col) {
                result += 1;
            }
        }
    }

    result
} 

fn update_grid2(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for row in 0..grid.len() {
        let mut row_chars = Vec::new();
        for col in 0..grid[0].len() {
            if grid[row][col] == '.' {
                row_chars.push('.');
                continue;
            }
            let count_occupied = count_neighbors2(grid, row, col);
            if grid[row][col] == 'L' {
                if count_occupied == 0 {
                    row_chars.push('#');
                } else {
                    row_chars.push('L');
                }
            } else if grid[row][col] == '#' {
                if count_occupied >= 5 {
                    row_chars.push('L');
                } else {
                    row_chars.push('#');
                }
            }
        }
        result.push(row_chars);
    }

    result
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let mut current_grid = grid.clone();
    loop {
        let new_grid = update_grid2(&current_grid);
        if new_grid == current_grid {
            return count_occupied(&current_grid)
        }
        current_grid = new_grid;
    }
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let grid = parse_grid(&contents);

    let result1 = part1(&grid);
    println!("Part 1 answer: {}", result1);

    let result2 = part2(&grid);
    println!("Part 2 answer: {}", result2);
}
