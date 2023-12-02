use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::fmt;
use std::fs;

struct RGB {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    pulls: Vec<RGB>,
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R: {}, G: {}, B: {}", self.red, self.green, self.blue)
    }
}

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"Game (\d+): (.*)").unwrap();
}

fn parse_pull(str: &str) -> RGB {
    let mut red: i32 = 0;
    let mut green: i32 = 0;
    let mut blue: i32 = 0;
    for entry in str.split(", ") {
        let entry_split: Vec<_> = entry.split(" ").collect();
        let count = entry_split[0].parse::<i32>().unwrap();
        let color = entry_split[1];
        if color == "red" {
            red = count;
        }
        if color == "green" {
            green = count;
        }
        if color == "blue" {
            blue = count;
        }
    }
    RGB {
        red: red,
        green: green,
        blue: blue,
    }
}

fn parse_game(line: &str) -> Option<Game> {
    let cap = GAME_REGEX.captures(line).unwrap();
    let id = cap.get(1).map(|m| m.as_str())?.parse::<i32>().ok()?;
    let pulls_str = cap.get(2).map(|m| m.as_str())?;
    let pulls = pulls_str.split("; ").map(parse_pull).collect();
    Some(Game {
        id: id,
        pulls: pulls,
    })
}

fn parse_input() -> Vec<Game> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content.lines();
    lines.filter_map(parse_game).collect()
}

fn pull_is_possible(pull: &RGB, target: &RGB) -> bool {
    pull.red <= target.red && pull.green <= target.green && pull.blue <= target.blue
}

fn game_is_possible_with_replacement(game: &Game, target: &RGB) -> bool {
    for pull in &game.pulls {
        if !pull_is_possible(&pull, &target) {
            println!(
                "Game {} Not possible ({}) against ({})",
                game.id, pull, target
            );
        }
    }
    game.pulls
        .iter()
        .all(|pull| pull_is_possible(pull, &target))
}

fn game_power(game: &Game) -> i32 {
    let init_rgb = RGB {
        red: 0,
        green: 0,
        blue: 0,
    };
    let max_rgb = game.pulls.iter().fold(init_rgb, |acc, next| RGB {
        red: max(acc.red, next.red),
        green: max(acc.green, next.green),
        blue: max(acc.blue, next.blue),
    });
    max_rgb.red * max_rgb.blue * max_rgb.green
}

fn main() {
    let games = parse_input();
    println!("{} games", games.len());
    let target = RGB {
        red: 12,
        green: 13,
        blue: 14,
    };
    let possible_with_replacement: i32 = games
        .iter()
        .filter(|game| game_is_possible_with_replacement(&game, &target))
        .map(|game| game.id)
        .sum();
    let power: i32 = games.iter().map(game_power).sum();
    println!("{} possible with replacement", possible_with_replacement);
    println!("{} total power", power)
}
