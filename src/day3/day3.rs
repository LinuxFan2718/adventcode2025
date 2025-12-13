use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "src/day3/test.txt";
    // let path = "src/day3/input.txt";
    // Open the file
    let file = File::open(&path)
        .unwrap_or_else(|e| panic!("Could not open {}: {}", path, e));

    // Wrap it in a buffered reader
    let reader = io::BufReader::new(file);

    let mut answers: Vec<String> = vec![];
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
        let mut best: Option<(usize, u32)> = None; // (byte_index, digit_value)

        for (i, c) in body.char_indices() {
            if let Some(d) = c.to_digit(10) {
                match best {
                    None => best = Some((i, d)),
                    Some((_, best_d)) if d > best_d => best = Some((i, d)),
                    Some((_, best_d)) if d == best_d => {} // keep leftmost
                    _ => {}
                }
            }
        }
        let mut tail = "";
        if let Some((idx, _digit)) = best {
            if idx + 1 <= text.len() {
                tail = &text[idx + 1..]; // from after the max digit to the end
                
            }
        }
        let max_digit = tail
            .chars()
            .filter_map(|c| c.to_digit(10))
            .max()
            .unwrap_or(0); // 0 if tail is empty

        if let Some((_idx, digit)) = best {
            let s = format!("{}{}", digit, max_digit);
            answers.push(s);
        }


        // if let Some((idx, digit)) = best {
        //     println!("max digit {} at byte index {}", digit, idx);
        // }

        // println!("{} {}\n", text, body);
      }
      // println!("{}", answers.join(", "));
      // returns Result<i64, _> so you can handle parse errors
      let sum: i64 = answers
          .iter()
          .map(|s| s.parse::<i64>())
          .collect::<Result<Vec<_>, _>>()?
          .iter()
          .sum();
      println!("sum = {}", sum);

      Ok(())
    }
