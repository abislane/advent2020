use std::collections::HashSet;
use std::env;
use std::fs;

struct Input {
    width: i32,
    height: i32,
    cubes: HashSet<(i32, i32)>, 
}

struct Area3d {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

struct Area4d {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
    w: (i32, i32)
}

fn parse_input(contents: &str) -> Input {
    let lines: Vec<&str> = contents.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    
    let mut cubes = HashSet::new();
    for y in  0..height {
        let chars: Vec<char> = lines[y].chars().collect();
        for x in 0..width {
            if chars[x] == '#' {
                cubes.insert((x as i32, y as i32));
            }
        }
    }

    Input { width: width as i32, height: height as i32, cubes }
}

fn get_neighbors(point: (i32, i32, i32), cubes: &HashSet<(i32, i32, i32)>) -> usize {
    let mut result = 0;
    for x in point.0-1..point.0+2 {
        for y in point.1-1..point.1+2 {
            for z in point.2-1..point.2+2 {
                let neighbor = (x, y, z);
                if neighbor != point && cubes.contains(&neighbor) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn update_cubes(cubes: &HashSet<(i32, i32, i32)>, area: &Area3d) -> HashSet<(i32, i32, i32)> {
    let mut updated_cubes = HashSet::new();
    for x in area.x.0..area.x.1 {
        for y in area.y.0..area.y.1 {
            for z in area.z.0..area.z.1 {
                let point = (x, y, z);
                let neighbors = get_neighbors(point, cubes);
                if (cubes.contains(&point) && (neighbors == 2 || neighbors == 3)) ||
                   (!cubes.contains(&point) && neighbors == 3) {
                    updated_cubes.insert(point);
                }
            }
        }
    }
    updated_cubes
}

fn part1(input: &Input) -> usize {
    let mut cubes = HashSet::new();
    for cube in &input.cubes {
        cubes.insert((cube.0, cube.1, 0));
    }
    let mut area = Area3d { x: (0, input.width), y: (0, input.height), z: (0, 1)};

    for _ in 0..6 {
        area = Area3d { 
            x: (area.x.0 - 1, area.x.1 + 1),
            y: (area.y.0 - 1, area.y.1 + 1),
            z: (area.z.0 - 1, area.z.1 + 1)
        };
        cubes = update_cubes(&cubes, &area);
    }

    cubes.len()
}

fn get_neighbors_4d(point: (i32, i32, i32, i32), cubes: &HashSet<(i32, i32, i32, i32)>) -> usize {
    let mut result = 0;
    for x in point.0-1..point.0+2 {
        for y in point.1-1..point.1+2 {
            for z in point.2-1..point.2+2 {
                for w in point.3-1..point.3+2 {
                    let neighbor = (x, y, z, w);
                    if neighbor != point && cubes.contains(&neighbor) {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn update_cubes_4d(cubes: &HashSet<(i32, i32, i32, i32)>, area: &Area4d) -> HashSet<(i32, i32, i32, i32)> {
    let mut updated_cubes = HashSet::new();
    for x in area.x.0..area.x.1 {
        for y in area.y.0..area.y.1 {
            for z in area.z.0..area.z.1 {
                for w in area.w.0..area.w.1 {
                    let point = (x, y, z, w);
                    let neighbors = get_neighbors_4d(point, cubes);
                    if (cubes.contains(&point) && (neighbors == 2 || neighbors == 3)) ||
                    (!cubes.contains(&point) && neighbors == 3) {
                        updated_cubes.insert(point);
                    }
                }
            }
        }
    }
    updated_cubes
}

fn part2(input: &Input) -> usize {
    let mut cubes = HashSet::new();
    for cube in &input.cubes {
        cubes.insert((cube.0, cube.1, 0, 0));
    }
    let mut area = Area4d { x: (0, input.width), y: (0, input.height), z: (0, 1), w: (0, 1)};

    for _ in 0..6 {
        area = Area4d { 
            x: (area.x.0 - 1, area.x.1 + 1),
            y: (area.y.0 - 1, area.y.1 + 1),
            z: (area.z.0 - 1, area.z.1 + 1),
            w: (area.w.0 - 1, area.w.1 + 1)
        };
        cubes = update_cubes_4d(&cubes, &area);
    }

    cubes.len()
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
