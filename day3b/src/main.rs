use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let sacks = input.lines();
    let mut sum: u32 = 0;

    let mut first_set = HashSet::new();
    let mut second_set = HashSet::new();
    let mut third_set = HashSet::new();

    let mut i = 0;
    for sack in sacks {
        match i {
            0 => {
                for char in sack.chars() {
                    first_set.insert(char);
                }
                i += 1;
            },
            1 => {
                for char in sack.chars() {
                    second_set.insert(char);
                }
                i += 1;
            },
            2 => {
                for char in sack.chars() {
                    third_set.insert(char);
                }
                i += 1;
            },
            _ => {
                i = 0;

                for badge in first_set.intersection(&second_set) {
                    if third_set.contains(badge) {
                        sum += compute_priority(*badge);
                    }
                }

                first_set.clear();
                second_set.clear();
                third_set.clear();
            }
        }
    }
    println!("Sum {sum}");
}

fn compute_priority(c: char) -> u32 {
    match c as u8 {
        b'a'..=b'z' => (1 + c as u8 - b'a') as u32,
        b'A'..=b'Z' => (27 + c as u8 - b'A') as u32,
        _ => unreachable!(),
    }
}
