use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[allow(dead_code)]
fn validate_a(input: &str) -> i32 {
    let input = input.trim();
    if input.is_empty() {
        return 0;
    }
    let mut first_jolt: i32 = input.chars().next().unwrap().to_digit(10).unwrap() as i32;
    let mut second_jolt: i32 = 0;
    let inputlen = input.chars().count();

    for (index, chars) in input.chars().enumerate().skip(1) {
        let current_jolt = chars.to_digit(10).unwrap() as i32;
        if current_jolt > first_jolt && index < inputlen - 1 {
            first_jolt = current_jolt;
            second_jolt = input.chars().nth(index + 1).unwrap().to_digit(10).unwrap() as i32;
        } else if current_jolt > second_jolt {
            second_jolt = current_jolt;
        }
    }
    return (first_jolt * 10) + second_jolt as i32;
}

fn validate_b(input: &str) -> u64 {
    let input = input.trim();
    let joltagelen = 12;
    if input.len() < joltagelen {
        return 0;
    }

    // We want to select k digits such that the resulting number is maximized.
    // This is equivalent to removing (len - k) digits to maximize the result.
    // Greedy strategy: iterate through digits, maintain a stack.
    // If current digit > stack.top and we have enough remaining digits to fill the rest, pop stack.

    let mut drop_count = input.len() - joltagelen;
    let mut result: Vec<u32> = Vec::with_capacity(joltagelen);

    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            while drop_count > 0 {
                if let Some(&last) = result.last() {
                    if last < digit {
                        result.pop();
                        drop_count -= 1;
                        continue;
                    }
                }
                break;
            }
            result.push(digit);
        }
    }

    // If we didn't drop enough (e.g. descending order), truncate the end
    result.truncate(joltagelen);

    let mut total: u64 = 0;
    for (i, &digit) in result.iter().enumerate() {
        total += (digit as u64) * 10u64.pow((joltagelen - 1 - i) as u32);
    }
    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input: String = if args.len() > 1 {
        let file_path = &args[1];
        read_file(file_path).expect("Failed to read file")
    } else {
        "987654321111111
        811111111111119
        234234234234278
        818181911112111"
            .to_string()
    };
    println!("Input: {}", &input);
    let mut sum = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let val = validate_b(&line);
        println!("Line value: {}", val);
        sum += val;
    }
    println!("Total sum: {}", sum);
}
