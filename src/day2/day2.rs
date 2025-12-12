use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // let path: &'static str = "src/day2/test.txt";
    let path = "src/day2/input.txt";  
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
              'outer: for i in l..=r {
                let num = i.to_string();
                let len: usize = num.len();
                for j  in 1..=len/2 {
                  let chunks: Vec<&str> = num
                    .as_bytes()
                    .chunks(j)
                    .map(|c| std::str::from_utf8(c).unwrap())
                    .collect();
                  let all_same = chunks.iter().all(|&c| c == chunks[0]);
                  if all_same {
                    sum += i;
                    println!("{}", i);
                    continue 'outer;
                  }

                }

              }
          }
        }
    }
    println!("sum={}", sum); // test=1227775554
    Ok(())
}
