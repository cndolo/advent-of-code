use std::fs::File;
use std::io::{BufRead, BufReader};

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

// Reads the input and returns two vectors - one for each each direction
fn read_input_from_file(file_path: &str) -> (Vec<i64>, Vec<i64>) {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);

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

fn main() {
    let (horizontal, vertical) = read_input_from_file("input.txt");
    let horiz_pos: i64 = horizontal.iter().sum();
    let vert_pos: i64 = vertical.iter().sum();
    println!("Final position: {}", horiz_pos * vert_pos);
}
