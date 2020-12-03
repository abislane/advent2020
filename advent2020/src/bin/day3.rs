use std::env;
use std::fs;

struct Grid {
    rows: usize,
    tree_grid: Vec<Vec<bool>>
}

fn parse_grid(contents: &str) -> Grid {
    let mut rows = 0;
    let mut tree_grid = Vec::new();
    for row in contents.lines() {
        let mut tree_row = Vec::new();
        for point in row.chars() {
            if point == '#' {
                tree_row.push(true);
            } else {
                tree_row.push(false);
            }
        }
        tree_grid.push(tree_row);
        rows += 1;
    }
    Grid {rows, tree_grid}
}

fn count_collisions(grid: &Grid, row_delta: usize, col_delta: usize) -> i64 {
    let mut cur_row = 0;
    let mut cur_col = 0;

    let mut collisions = 0;
    while cur_row < grid.rows {
        let mod_col = cur_col % grid.tree_grid[cur_row].len(); 
        if grid.tree_grid[cur_row][mod_col] {
            collisions += 1;
        }
        cur_row += row_delta;
        cur_col += col_delta;
    }
    collisions
}

fn part1(grid: &Grid) -> i64 {
    count_collisions(grid, 1, 3)
}

fn part2(grid: &Grid) -> i64 {
    let mut result: i64 = count_collisions(grid, 1, 1);
    result *= count_collisions(grid, 1, 3);
    result *= count_collisions(grid, 1, 5);
    result *= count_collisions(grid, 1, 7);
    result *= count_collisions(grid, 2, 1);
    result
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
