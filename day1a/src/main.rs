use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut most: u32 = 0;
    let mut count: u32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(calories_string) = line {
                if calories_string.is_empty() {
                    if count > most {
                        most = count;
                    }
                    count = 0;
                } else {
                    count += calories_string.trim().parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("Most calories: {most}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}