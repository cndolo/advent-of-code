use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).expect("file not found");
    BufReader::new(file)
}

// Returns 0 if the instruction is along the x axis and 1 otherwise
fn parse_input(line: Vec<&str>) -> (bool, i64) {
    let unit = line[1].parse::<i64>().unwrap();
    match line[0] {
        "forward" => (false, unit),
        "down" => (true, unit),
        "up" => (true, -unit),
        _ => panic!("unknown"),
    }
}

// down X increases your aim by X units.
fn apply_down(down: i64, curr_aim: i64) -> i64 {
    curr_aim + down
}

// up X decreases your aim by X units.
fn apply_up(up: i64, curr_aim: i64) -> i64 {
    curr_aim - up
}

// forward X does two things:
//  - It increases your horizontal position by X units.
//  - It increases your depth by your aim multiplied by X.
fn apply_forward(
    forward: i64,
    curr_horizontal: i64,
    curr_aim: i64,
    curr_vertical: i64,
) -> (i64, i64) {
    let horizontal = forward + curr_horizontal;
    let depth = curr_vertical + forward * curr_aim;
    (horizontal, depth)
}
// Reads the input and returns two vectors - one for each each direction
fn compute_part_one(file_path: &str) -> (Vec<i64>, Vec<i64>) {
    let reader = read_input_from_file(file_path);
    let mut horizontal: Vec<i64> = vec![];
    let mut vertical: Vec<i64> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let instructions: Vec<&str> = line.split_whitespace().collect();
        let (direction, unit) = parse_input(instructions.clone());
        if direction {
            vertical.push(unit);
        } else {
            horizontal.push(unit);
        }
    }
    (horizontal, vertical)
}

// Reads and parses input.
// Returns the horizontal and vertical position as a tuple
fn compute_part_two(file_path: &str) -> (i64, i64) {
    let reader = read_input_from_file(file_path);
    let mut aim = 0;
    let mut curr_horizontal = 0;
    let mut curr_vertical = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let instructions: Vec<&str> = line.split_whitespace().collect();
        let unit = instructions[1].parse::<i64>().unwrap();
        match instructions[0] {
            "forward" => {
                let new_position = apply_forward(unit, curr_horizontal, aim, curr_vertical);
                curr_horizontal = new_position.0;
                curr_vertical = new_position.1;
            }
            "down" => aim = apply_down(unit, aim),
            "up" => aim = apply_up(unit, aim),
            _ => panic!("unknown"),
        }
    }
    (curr_horizontal, curr_vertical)
}

fn main() {
    let (horizontal, vertical) = compute_part_one("input.txt");
    let horiz_pos: i64 = horizontal.iter().sum();
    let vert_pos: i64 = vertical.iter().sum();
    println!("Part 1 final position: {}", horiz_pos * vert_pos);
    let (horizontal, vertical) = compute_part_two("input.txt");
    println!("Part 2 final position: {}", horizontal * vertical);
}
