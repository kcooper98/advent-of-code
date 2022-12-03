fn main() {
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;
    let mut count: u32 = 0;
    let lines = include_str!("../input.txt").lines();

    for line in lines {
        if line.is_empty() {
            if count > first {
                third = second;
                second = first;
                first = count;
            } else if count > second {
                third = second;
                second = count;
            } else if count > third {
                third = count;
            }
            count = 0;
        } else {
            count += line.trim().parse::<u32>().unwrap();
        }
    }
    let sum = first + second + third;
    println!("First: {first}, second: {second}, third: {third}, sum: {sum}");
}
