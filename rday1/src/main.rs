use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn read_file(file_path: &str) -> std::io::Result<Vec<String>> {
    let mut list: Vec<String> = Vec::new();
    let file = File::open(file_path).expect("File not found");
    let buf = BufReader::new(file);

    for line in buf.lines() {
        list.push(line?);
    }
    Ok(list)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = if args.len() > 1 {
        &args[1]
    } else {
        "example.txt"
    };

    let start = 50;
    let lines = read_file(file).expect("Failed to read file");
    println!("Lines: {:?}", lines);
    let result = input(start, lines);
    println!("Result: {}", result);
}

fn rotate(cur: i32, amt: i32, is_left: bool) -> (i32, i32) {
    let mut new = cur;
    let mut zeroed = 0;
    if is_left {
        new -= amt;
    } else {
        new += amt;
    }
    if new > 99 {
        new -= 100;
        if cur != 0 && new != 0 {
            zeroed += 1;
        }
    }
    if new < 0 {
        new += 100;
        if cur != 0 && new != 0 {
            zeroed += 1;
        }
    }

    if new == 0 {
        zeroed += 1;
    }

    return (new, zeroed);
}

// fn inputA(rotations: vec<&str>) -> i32 {
fn input(start: i32, rotations: Vec<String>) -> i32 {
    let mut counter = 0;
    let mut result = start;
    println!("Start: {}", start);
    for rot in rotations {
        let mut is_left = true;
        print!("Line: {}, ", rot);
        let amt: i32 = rot[1..].parse().unwrap();
        let overclicks = amt / 100;
        let modamt = amt - (overclicks * 100);

        if rot.to_uppercase().starts_with("R") {
            is_left = false;
        }
        let (new, zeroed) = rotate(result, modamt, is_left);
        if overclicks + zeroed > 0 {
            print!("Hit zero ({}, {}) times, ", zeroed, overclicks);
        }
        counter += zeroed + overclicks;
        result = new;
        println!("Result: {}", result);
    }
    return counter;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_input_a() {
//         assert_eq!(inputA(50, vec!["L68".to_string()]), 18);
//         assert_eq!(inputA(50, vec!["R68".to_string()]), 18);
//     }
// }
