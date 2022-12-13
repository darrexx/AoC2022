// A Rock B Paper C Scissors
// X Rock Y Paper Z Scissors

// X = 1, y=2, Z = 3
// lose 0, draw 3, win 6

fn main() {
    let input = include_str!("../input.txt");
    let rounds = input
        .lines()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    println!("Part1: {:#?}", part1(rounds.clone()));
    println!("Part2: {:#?}", part2(rounds))
}

enum MatchResult {
    Win,
    Lose,
    Draw,
}

impl MatchResult {
    fn get_score(&self) -> usize {
        match self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Lose => 0,
        }
    }

    fn match_symbol_to_result(symbol: &str) -> Self {
        match symbol {
            "X" => MatchResult::Lose,
            "Y" => MatchResult::Draw,
            "Z" => MatchResult::Win,
            _ => MatchResult::Lose,
        }
    }
}

fn part1(rounds: Vec<Vec<&str>>) -> usize {
    rounds
        .into_iter()
        .map(|round| calculate_winner(round[0], round[1]).get_score() + get_symbol_value(round[1]))
        .sum()
}

fn calculate_winner(opponent: &str, you: &str) -> MatchResult {
    match (opponent, you) {
        ("A", "X") => MatchResult::Draw,
        ("A", "Y") => MatchResult::Win,
        ("A", "Z") => MatchResult::Lose,
        ("B", "X") => MatchResult::Lose,
        ("B", "Y") => MatchResult::Draw,
        ("B", "Z") => MatchResult::Win,
        ("C", "X") => MatchResult::Win,
        ("C", "Y") => MatchResult::Lose,
        ("C", "Z") => MatchResult::Draw,
        _ => MatchResult::Lose,
    }
}

fn get_symbol_value(you: &str) -> usize {
    match you {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn part2(rounds: Vec<Vec<&str>>) -> usize {
    rounds
        .into_iter()
        .map(|round| {
            MatchResult::match_symbol_to_result(round[1]).get_score()
                + get_symbol_value(calculate_turn(
                    round[0],
                    MatchResult::match_symbol_to_result(round[1]),
                ))
        })
        .sum()
}

fn calculate_turn(opponent: &str, result: MatchResult) -> &str {
    match (opponent, result) {
        ("A", MatchResult::Win) => "Y",
        ("A", MatchResult::Draw) => "X",
        ("A", MatchResult::Lose) => "Z",
        ("B", MatchResult::Win) => "Z",
        ("B", MatchResult::Draw) => "Y",
        ("B", MatchResult::Lose) => "X",
        ("C", MatchResult::Win) => "X",
        ("C", MatchResult::Draw) => "Z",
        ("C", MatchResult::Lose) => "Y",
        _ => "X",
    }
}
