// focus on readability and function use, redundant in use of lose func but reasonable
fn main() {
    assert_eq!(translate_opponent('A'), "rock");
    assert_eq!(translate_opponent('B'), "paper");
    assert_eq!(translate_opponent('C'), "scissors");
    assert_eq!(translate_player('X'), "rock");
    assert_eq!(translate_player('Y'), "paper");
    assert_eq!(translate_player('Z'), "scissors");

    println!("Total score: {}", play_game());
}

fn play_game() -> u32 {
    let mut score = 0;
    for line in include_str!("../input.txt").lines() {
        if !line.is_empty() {
            score += play_round(translate_player(line.chars().nth(2).unwrap()), translate_opponent(line.chars().nth(0).unwrap()));
        }
    }

    return score;
}

fn play_round(player: &str, opponent: &str) -> u32 {
    let mut score = 0;
    match player {
        "rock" => {
            score += 1;
            match opponent {
                "rock" => score += draw(),
                "scissors" => score += win(),
                "paper" => score += lose(),
                &_ => todo!(),
            }
        },
        "paper" => {
            score += 2; 
            match opponent {
                "rock" => score += win(),
                "scissors" => score += lose(),
                "paper" => score += draw(),
                &_ => todo!(),
            }
        },
        "scissors" => {
            score += 3;
            match opponent {
                "rock" => score += lose(),
                "scissors" => score += draw(),
                "paper" => score += win(),
                &_ => todo!(),
            }
        },
        &_ => todo!(),
    };

    return score
}

fn translate_opponent(c: char) -> &'static str {
    match c {
        'A' => return "rock",
        'B' => return "paper",
        'C' => return "scissors",
        _ => panic!("failed to translate input"),
    };
}

fn translate_player(c: char) -> &'static str {
    match c {
        'X' => return "rock",
        'Y' => return "paper",
        'Z' => return "scissors",
        _ => panic!("failed to translate output"),
    };
}

fn win() -> u32 {
    return 6;
}

fn draw() -> u32 {
    return 3;
}

fn lose() -> u32 {
    return 0;
}
