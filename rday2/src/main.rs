use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input: String = if args.len() > 1 {
        let file_path = &args[1];
        read_file(file_path).expect("Failed to read file")
    } else {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124"
            .to_string()
    };

    print!("Count: {}", input_a(&input));
}

fn split_input(input: &str, char: char) -> Vec<&str> {
    return input.split(char).map(|l| l.trim()).collect();
}

fn is_invalid(num: u64) -> bool {
    let numstr = num.to_string();
    let len = numstr.len();

    if len > 1 && numstr.starts_with('0') {
        return false;
    }

    for tok in 1..=len / 2 {
        if len % tok == 0 {
            let pat = &numstr[0..tok];
            let num_rep = len / tok;

            // We are taking the length of the current number in range
            // then we are going to take our pattern (based on token length
            // substring), then we push that token repeatingly until length
            // of input num.
            // ie, len 6, tok 2, pat 32, rep_string 323232
            let mut rep_string = String::with_capacity(len);
            for _ in 0..num_rep {
                rep_string.push_str(pat);
            }

            // we then compare our generated length string against the
            // original input string
            if numstr == rep_string {
                println!("Found: {}", rep_string);
                return true;
            }
        }
    }
    return false;
}

fn input_a(input: &str) -> u64 {
    let mut invalids = Vec::new();
    for code in split_input(input, ',') {
        let start = split_input(code, '-')[0]
            .parse::<u64>()
            .expect("Failed to parse start");
        let end = split_input(code, '-')[1]
            .parse::<u64>()
            .expect("Failed to parse end");

        for num in start..=end {
            if (is_invalid(num)) {
                invalids.push(num);
            }
        }
    }
    println!("Invalids: {:?}", invalids);
    return invalids.iter().sum();
}
