// focus on readability and function use, redundant in use of lose func but reasonable
fn main() {
    println!("Total score: {}", play_game());
}

fn play_game() -> u32 {
    let mut score = 0;
    for line in include_str!("../input.txt").lines() {
        if !line.is_empty() {
            score += play_round(translate_player(line.chars().nth(0).unwrap()), translate_outcome(line.chars().nth(2).unwrap()));
        }
    }

    return score;
}

fn play_round(player: &str, outcome: &str) -> u32 {
    let mut score = 0;
    match outcome {
        "lose" => {
            score += lose();
            match player {
                "rock" => score += 3,
                "paper" => score += 1,
                "scissors" => score += 2,
                &_ => todo!(),
            };
        },
        "draw" => {
            score += draw();
            match player {
                "rock" => score += 1,
                "paper" => score += 2,
                "scissors" => score += 3,
                &_ => todo!(),
            };
        },
        "win" => {
            score += win();
            match player {
                "rock" => score += 2,
                "paper" => score += 3,
                "scissors" => score += 1,
                &_ => todo!(),
            };
        },
        &_ => todo!(),
    };

    return score
}

fn translate_outcome(c: char) -> &'static str {
    match c {
        'X' => return "lose",
        'Y' => return "draw",
        'Z' => return "win",
        _ => panic!("failed to translate input"),
    };
}

fn translate_player(c: char) -> &'static str {
    match c {
        'A' => return "rock",
        'B' => return "paper",
        'C' => return "scissors",
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
