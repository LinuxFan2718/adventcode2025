use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "day1/test1.txt";

    // Open the file
    let file = File::open(&path)
        .unwrap_or_else(|e| panic!("Could not open {}: {}", path, e));

    // Wrap it in a buffered reader
    let reader = io::BufReader::new(file);

    // initialize the dial
    let mut dial: i32 = 50;
    let mut sign: i32;
    let mut times_dial_at_zero: i32 = 0;

    // Iterate over the lines
    for line_result in reader.lines() {
        // First, handle I/O errors
        let text = match line_result {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };

        // ---- HERE is where we work with `text` ----

        // 1) Make sure there's at least one character
        let first_char = match text.chars().next() {
            Some(c) => c,
            None => {
                eprintln!("Encountered an empty line");
                continue;
            }
        };

        // 2) Update sign based on the first char
        match first_char {
            'L' => sign = -1,
            'R' => sign = 1,
            other => {
                eprintln!("First char not L or R: {}", other);
                continue;
            }
        }

        // 3) Take the rest of the line after the first character
        //    This is fine for AoC-style ASCII input.
        let rest = &text[1..];

        // 4) Parse the rest as an integer
        let value: i32 = match rest.trim().parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Could not parse number from {:?}: {}", rest, e);
                continue;
            }
        };

        // 5) Use sign and value to update dial
        let change: i32 = sign * value;
        dial += change;
        if dial < 0 {
            dial += 100;
        }
        dial = dial % 100;
        if dial == 0 {
            times_dial_at_zero += 1;
        }
        println!("Line: {:?}, sign: {}, value: {}, dial: {} tdaz: {}", text, sign, value, dial, times_dial_at_zero);
    }

    println!("Final dial: {}", dial);
    println!("Times dial at zero {}", times_dial_at_zero);

    Ok(())
}
