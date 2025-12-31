use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let path = "src/day3/test.txt";
    let path = "src/day3/input.txt";
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

        let mut digits = vec![];
        let mut current_pos = 0;

        // Extract 12 digits, ensuring enough digits remain for future iterations
        for iteration in 0..12 {
            let remaining_needed = 12 - iteration - 1; // digits needed after this iteration

            // For first iteration, search in body; for others, search in text
            let (search_text, search_len) = if iteration == 0 {
                (body, body.len())
            } else {
                (text.as_str(), text.len())
            };

            if current_pos >= search_len {
                break;
            }

            // Calculate the maximum position we can search to (must leave enough digits for later)
            let max_search_pos = if search_len > remaining_needed {
                search_len - remaining_needed
            } else {
                search_len
            };

            // Find max digit in the constrained range
            let mut best: Option<(usize, u32)> = None;
            for (i, c) in search_text.char_indices() {
                if i < current_pos || i >= max_search_pos {
                    continue;
                }
                if let Some(d) = c.to_digit(10) {
                    match best {
                        None => best = Some((i, d)),
                        Some((_, best_d)) if d > best_d => best = Some((i, d)),
                        Some((_, best_d)) if d == best_d => {} // keep leftmost
                        _ => {}
                    }
                }
            }

            if let Some((idx, digit)) = best {
                digits.push(digit);
                current_pos = idx + 1;
            } else {
                break;
            }
        }

        if !digits.is_empty() {
            let s = digits.iter().map(|d| d.to_string()).collect::<String>();
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
