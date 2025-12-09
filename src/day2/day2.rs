use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path: &'static str = "src/day2/test.txt";
    // let path = "src/day2/input.txt";  
    let file = File::open(&path)
        .unwrap_or_else(|e| panic!("Could not open {}: {}", path, e));
    // Wrap it in a buffered reader
    let reader = io::BufReader::new(file);
    let mut sum: i64 = 0;

    for line in reader.lines() {
        let line = line?;
        for value in line.split(',').map(str::trim).filter(|v| !v.is_empty()) {
          if let Some((left, right)) = value.split_once('-') {
              let l: i64 = left.parse().unwrap();
              let r: i64 = right.parse().unwrap();
              for i in l..=r {
                let num = i.to_string();
                let len: usize = num.len();
                if len % 2 == 0 {
                  let (first_half, second_half) = num.split_at(len / 2);
                  if first_half == second_half {
                    sum += i;
                  }
                }
              }
          }
        }
    }
    println!("sum={}", sum); // test=1227775554
    Ok(())
}
