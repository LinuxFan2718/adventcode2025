use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "src/day3/test.txt";
    // let path = "src/day3/input.txt";
    // Open the file
    let file = File::open(&path)
        .unwrap_or_else(|e| panic!("Could not open {}: {}", path, e));

    // Wrap it in a buffered reader
    let reader = io::BufReader::new(file);
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
        let body = text
            .char_indices()
            .next_back()
            .map(|(idx, _)| &text[..idx])
            .unwrap_or(""); // empty if text is empty

        println!("{} {}", text, body);
      }
      Ok(())
    }