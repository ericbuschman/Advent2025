use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn convert_vector2(text: &str) -> Vec<(u32, u32)> {
    text.lines()
        .for_each(|line| {
            let y = 0;
            let mut chars = line.chars();
            
            let x = chars.next().unwrap();
            let y = chars.next().unwrap();
            y+=1;
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input: Vec<(u32, u32)> = if args.len() > 1 {
        let file_path = &args[1];
        let filetext = read_file(file_path).expect("Failed to read file");
        convert_vector2(&filetext);
    } else {
        convert_vector2(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .to_string(),
        );
    };
    println!("Input: {}", &input);
}
