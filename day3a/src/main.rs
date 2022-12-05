use std::collections::HashSet;

fn main() {
    let sack = include_str!("../input.txt");
    let lines = sack.lines();
    let mut sum: u32 = 0;
    
    for line in lines {
        let mut first_set = HashSet::new();
        let mut second_set = HashSet::new();
        let first_half = line[0..line.len()/2].to_string();
        let second_half = line[line.len()/2..line.len()].to_string();

        for char in first_half.chars() {
            first_set.insert(char);
        }
        for char in second_half.chars() {
            second_set.insert(char);
        }

        for c in first_set {
            if second_set.contains(&c) {
                sum += compute_priority(c);
            }
        }
    }

    println!("Priority sum: {sum}");
}

fn compute_priority(c: char) -> u32 {
    match c as u8 {
        b'a'..=b'z' => (1 + c as u8-b'a') as u32,
        b'A'..=b'Z' => (27 + c as u8 -b'A') as u32,
        _ => unreachable!(),
    }
}
