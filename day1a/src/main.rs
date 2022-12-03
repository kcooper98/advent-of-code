fn main() {
    let mut most: u32 = 0;
    let mut count: u32 = 0;
    let lines = include_str!("../input.txt").lines();

    for line in lines {
        if line.is_empty() {
            if count > most {
                most = count;
            }
            count = 0;
        } else {
            count += line.trim().parse::<u32>().unwrap();
        }
    }
    println!("Most calories: {most}");
}