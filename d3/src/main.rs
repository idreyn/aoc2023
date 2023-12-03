use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

lazy_static! {
    static ref PART_NUMBER_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Part {
    x: i32,
    y: i32,
    symbol: char,
}

#[derive(Debug)]
struct Number {
    x: RangeInclusive<i32>,
    y: i32,
    val: i32,
}

#[derive(Debug)]
struct PartNumber<'a> {
    part: &'a Part,
    number: &'a Number,
}

fn is_part(chr: char) -> bool {
    chr != '.' && !chr.is_ascii_digit()
}

fn find_parts(schematic: &Vec<&str>) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    for (y, line) in schematic.iter().enumerate() {
        for (x, char) in line.chars().into_iter().enumerate() {
            if is_part(char) {
                parts.push(Part {
                    x: x as i32,
                    y: y as i32,
                    symbol: char,
                });
            }
        }
    }
    parts
}

fn find_numbers(schematic: &Vec<&str>) -> Vec<Number> {
    let mut part_numbers: Vec<Number> = Vec::new();
    for (y, line) in schematic.iter().enumerate() {
        for cap in PART_NUMBER_REGEX.find_iter(line) {
            let cap_range = cap.range();
            let part_number = Number {
                x: (cap_range.start as i32)..=((cap_range.end - 1) as i32),
                y: y as i32,
                val: cap.as_str().parse::<i32>().unwrap(),
            };
            part_numbers.push(part_number);
        }
    }
    part_numbers
}

fn find_part_numbers<'a>(parts: &'a Vec<Part>, numbers: &'a Vec<Number>) -> Vec<PartNumber<'a>> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    for number in numbers {
        let x_range = (number.x.start() - 1)..=(number.x.end() + 1);
        let y_range = (number.y - 1)..=(number.y + 1);
        if let Some(part) = parts
            .iter()
            .find(|part| x_range.contains(&part.x) && y_range.contains(&part.y))
        {
            part_numbers.push(PartNumber { part, number });
        }
    }
    part_numbers
}

fn part_one(part_numbers: &Vec<PartNumber>) {
    let sum_of_part_numbers: i32 = part_numbers.iter().map(|pn| pn.number.val).sum();
    println!("Sum of part numbers is {}", sum_of_part_numbers);
}

fn part_two(part_numbers: &Vec<PartNumber>) {
    let mut numbers_by_part: HashMap<&Part, Vec<&Number>> = HashMap::new();
    for part_number in part_numbers {
        let &PartNumber { part, number } = part_number;
        if numbers_by_part.get(part).is_none() {
            numbers_by_part.insert(part, Vec::new());
        }
        numbers_by_part.get_mut(part).unwrap().push(number);
    }
    let sum_of_gear_ratios: i32 = numbers_by_part
        .iter()
        .filter_map(|(part, numbers)| {
            if part.symbol == '*' && numbers.len() == 2 {
                return Some(numbers[0].val * numbers[1].val);
            }
            None
        })
        .sum();
    println!("Sum of gear ratios is {}", sum_of_gear_ratios);
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let schematic: Vec<&str> = input.lines().collect();
    let parts = find_parts(&schematic);
    let numbers = find_numbers(&schematic);
    let part_numbers = find_part_numbers(&parts, &numbers);
    part_one(&part_numbers);
    part_two(&part_numbers);
}
