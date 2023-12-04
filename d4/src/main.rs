use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    your_numbers: Vec<i32>,
}

lazy_static! {
    static ref CARD_REGEX: Regex = Regex::new(r"Card \s*(\d+): ([\d\s]+) \| ([\d\s]+)").unwrap();
}

fn parse_int_series(series: &str) -> Vec<i32> {
    series
        .trim()
        .split_whitespace()
        .map(|el| el.trim().parse::<i32>().unwrap())
        .collect()
}

fn parse_to_card(line: &str) -> Card {
    if let Some(caps) = CARD_REGEX.captures(line) {
        let id = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let winning_numbers = parse_int_series(caps.get(2).unwrap().as_str());
        let your_numbers = parse_int_series(caps.get(3).unwrap().as_str());
        return Card {
            id,
            winning_numbers,
            your_numbers,
        };
    }
    panic!("Failed to parse card: {}", line);
}

fn card_value(card: &Card) -> i32 {
    card.your_numbers.iter().fold(0, |val, next| {
        let number_wins = card.winning_numbers.contains(next);
        if number_wins {
            return if val == 0 { 1 } else { val * 2 };
        }
        return val;
    })
}

fn count_matching(card: &Card) -> i32 {
    card.your_numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count() as i32
}

fn count_duplicate_cards(cards: &Vec<Card>) -> i32 {
    let mut counts: Vec<i32> = cards.iter().map(|_| 1).collect();
    for (idx, card) in cards.iter().enumerate() {
        let matches = count_matching(card);
        let current_count = counts[idx];
        for next_idx in (idx + 1)..(idx + 1 + matches as usize) {
            if next_idx < counts.len() {
                counts[next_idx] += current_count;
            }
        }
    }
    counts.iter().sum()
}

fn part_1(cards: &Vec<Card>) {
    let total_value: i32 = cards.iter().map(|c| card_value(&c)).sum();
    println!("Total card value: {}", total_value);
}

fn part_2(cards: &Vec<Card>) {
    let total = count_duplicate_cards(&cards);
    println!("Total duplicates: {}", total);
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let lines = contents.lines();
    let cards: Vec<_> = lines.map(parse_to_card).collect();
    part_1(&cards);
    part_2(&cards);
}
