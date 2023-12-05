use std::cmp::min;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct AlmanacRange {
    from_start: i64,
    to_start: i64,
    length: i64,
}

impl AlmanacRange {
    fn map(&self, val: i64) -> Option<i64> {
        let valid_range = self.from_start..=(self.from_start + self.length);
        match valid_range.contains(&val) {
            true => Some(self.to_start + (val - self.from_start)),
            false => None,
        }
    }
}

#[derive(Debug, Clone)]
struct AlmanacSection {
    from: String,
    to: String,
    ranges: Vec<AlmanacRange>,
}

impl AlmanacSection {
    fn map(&self, val: i64) -> i64 {
        let found = self.ranges.iter().find_map(|r| r.map(val));
        match found {
            Some(x) => x,
            None => val,
        }
    }
}

#[derive(Debug)]
struct Almanac {
    maps: HashMap<String, HashMap<String, AlmanacSection>>,
}

impl Almanac {
    fn get_available_mappings(&self, from: &String) -> Vec<&String> {
        match self.maps.get(from) {
            Some(h) => h.keys().collect(),
            None => Vec::new(),
        }
    }

    fn get_an_available_mapping(&self, from: &String) -> Option<&String> {
        let available = self.get_available_mappings(from);
        match available.as_slice() {
            [] => None,
            [to, ..] => Some(to),
        }
    }

    fn map(&self, from: &String, to: &String, val: i64) -> i64 {
        let section = self.maps.get(from).unwrap().get(to);
        match section {
            Some(sec) => sec.map(val),
            None => panic!("Almanac is missing mapping {} -> {}", from, to),
        }
    }
}

fn parse_almanac_section(src: &str) -> AlmanacSection {
    let mut by_line = src.split("\n");
    let mut ranges: Vec<AlmanacRange> = Vec::new();
    let from_to: Vec<&str> = by_line
        .next()
        .unwrap()
        .split(" ")
        .nth(0)
        .unwrap()
        .split("-")
        .collect();
    for line in by_line {
        let entries: Vec<_> = line
            .split(" ")
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();
        ranges.push(AlmanacRange {
            from_start: entries[1],
            to_start: entries[0],
            length: entries[2],
        })
    }
    AlmanacSection {
        from: from_to[0].to_string(),
        to: from_to[2].to_string(),
        ranges: ranges,
    }
}

fn make_almanac(sections: Vec<AlmanacSection>) -> Almanac {
    let mut maps: HashMap<String, HashMap<String, AlmanacSection>> = HashMap::new();
    for section in sections {
        let from = section.from.clone();
        let to = section.to.clone();
        let new_section = section.clone();
        maps.entry(from)
            .or_insert_with(HashMap::new)
            .insert(to, new_section);
    }
    Almanac { maps }
}

fn lowest_location_for_seed(seed: i64, almanac: &Almanac) -> i64 {
    let mut lowest = i64::MAX;
    let mut value: i64 = seed;
    let mut from: String = "seed".to_string();
    loop {
        let requested_mapping = almanac.get_an_available_mapping(&from);
        if let Some(to) = requested_mapping {
            let mapped = almanac.map(&from, to, value);
            value = mapped;
            from = to.clone();
        } else {
            assert!(from == "location");
            lowest = min(lowest, value);
            break;
        }
    }
    lowest
}

fn lowest_location_for_seeds(seeds: &Vec<i64>, almanac: &Almanac) -> i64 {
    seeds.iter().fold(i64::MAX, |acc, next| {
        min(acc, lowest_location_for_seed(*next, almanac))
    })
}

fn part_1(seeds: &Vec<i64>, almanac: &Almanac) {
    let lowest = lowest_location_for_seeds(seeds, almanac);
    println!("Lowest location for seeds: {}", lowest);
}

fn part_2(seeds: &Vec<i64>, almanac: &Almanac) {
    let all_seeds: Vec<_> = seeds
        .chunks(2)
        .map(|window| match window {
            [a, b] => Some(*a..(*a + *b)),
            _ => None,
        })
        .filter_map(|x| x)
        .map(|x| x.collect::<Vec<i64>>())
        .flatten()
        .collect();
    let lowest = lowest_location_for_seeds(&all_seeds, almanac);
    println!("Lowest location for seed ranges: {}", lowest);
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let sections: Vec<&str> = contents.split("\n\n").map(|s| s.trim()).collect();
    let seeds: Vec<i64> = sections[0]
        .split(" ")
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    let sections: Vec<_> = sections[1..]
        .iter()
        .map(|s| parse_almanac_section(s))
        .collect();
    let almanac = make_almanac(sections);
    lowest_location_for_seeds(&seeds, &almanac);
    part_1(&seeds, &almanac);
    part_2(&seeds, &almanac);
}
