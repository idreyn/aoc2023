use std::fs;

const SPELLED_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_digit(str: &str, use_spelled: bool) -> Option<i32> {
    if str.chars().nth(0)?.is_ascii_digit() {
        return Some(str[0..1].parse::<i32>().unwrap());
    }
    if use_spelled {
        for i in 0..SPELLED_DIGITS.len() {
            if str.starts_with(SPELLED_DIGITS[i]) {
                return Some(i.try_into().unwrap());
            }
        }
    }
    return None;
}

fn find_line(str: &str, use_spelled: bool) -> i32 {
    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;
    for i in 0..str.len() {
        let found = find_digit(&str[i..], use_spelled);
        first = match found {
            None => first,
            Some(f) => {
                if first.is_none() {
                    Some(f)
                } else {
                    first
                }
            }
        };
        last = match found {
            None => last,
            Some(f) => Some(f),
        };
    }
    let total: i32 = 10 * first.unwrap() + last.unwrap();
    total
}

fn find_sum(use_spelled: bool) {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content.lines();
    let sum = lines.fold(0, |acc, line| acc + find_line(line, use_spelled));
    println!("{}", sum);
}

fn main() {
    find_sum(false);
    find_sum(true);
}
