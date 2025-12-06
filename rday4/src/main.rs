use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn build_hashmap(input: &str) -> Option<(HashMap<usize, bool>, usize)> {
    let mut map = HashMap::new();
    let line_length = input.lines().next()?.len();
    let mut linecount = 0;
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let loc = i + (line_length * linecount);
            if c == '@' {
                map.insert(loc, true);
            }
        }
        linecount += 1;
    }
    Some((map, line_length))
}

fn solve_a(input: HashMap<usize, bool>, line_length: usize) {
    let mut count = 0;

    for (i, _) in input.iter() {
        let i = *i;
        let mut found_items = 0;
        let x = i % line_length;

        // check right
        if x < line_length - 1 {
            if *input.get(&(i + 1)).unwrap_or(&false) {
                found_items += 1;
            }
        }

        // check left
        if x > 0 {
            if let Some(key) = i.checked_sub(1) {
                if *input.get(&key).unwrap_or(&false) {
                    found_items += 1;
                }
            }
        }

        // check up
        if let Some(key) = i.checked_sub(line_length) {
            if *input.get(&key).unwrap_or(&false) {
                found_items += 1;
            }
        }

        // check up-right
        if x < line_length - 1 {
            if let Some(temp_key) = i.checked_sub(line_length) {
                if *input.get(&(temp_key + 1)).unwrap_or(&false) {
                    found_items += 1;
                }
            }
        }

        // check up-left
        if x > 0 {
            if let Some(temp_key) = i.checked_sub(line_length) {
                if let Some(key) = temp_key.checked_sub(1) {
                    if *input.get(&key).unwrap_or(&false) {
                        found_items += 1;
                    }
                }
            }
        }

        // check down
        if *input.get(&(i + line_length)).unwrap_or(&false) {
            found_items += 1;
        }

        // check down-left
        if x > 0 {
            if let Some(key) = (i + line_length).checked_sub(1) {
                if *input.get(&key).unwrap_or(&false) {
                    found_items += 1;
                }
            }
        }

        // check down-right
        if x < line_length - 1 {
            if *input.get(&(i + line_length + 1)).unwrap_or(&false) {
                found_items += 1;
            }
        }

        if found_items <= 3 {
            count += 1;
        }
    }

    println!("Number of <= 3 adjecent @: {}", count);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (input, line_length): (HashMap<usize, bool>, usize) = if args.len() > 1 {
        let file_path = &args[1];
        let filetext = read_file(file_path).expect("Failed to read file");
        build_hashmap(&filetext).expect("Could not build hashmap")
    } else {
        build_hashmap(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        )
        .expect("Could not build hashmap")
    };
    println!("Input: {:?}\nLine Length: {}", &input, line_length);
    solve_a(input, line_length);
}
