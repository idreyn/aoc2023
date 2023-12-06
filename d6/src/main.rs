use std::fs;

struct Race {
    allotted_time: i64,
    record_distance: i64,
}

fn can_win_race(race: &Race, hold_time: i64) -> bool {
    assert!(hold_time <= race.allotted_time);
    let travel_time = race.allotted_time - hold_time;
    let distance_traveled = travel_time * hold_time;
    distance_traveled > race.record_distance
}

fn ways_to_win_race(race: &Race) -> i64 {
    (0..=race.allotted_time)
        .map(|hold_time| can_win_race(race, hold_time) as i64)
        .sum()
}

fn part_1(races: &Vec<Race>) {
    let ways_to_win_each = races.iter().map(ways_to_win_race);
    let answer = ways_to_win_each.fold(1, |acc, next| acc * next);
    println!("Part 1: {}", answer);
}

fn part_2(races: &Vec<Race>) {
    let badly_kerned_race_time: String =
        races.iter().map(|r| r.allotted_time.to_string()).collect();
    let badly_kerned_race_distance: String = races
        .iter()
        .map(|r| r.record_distance.to_string())
        .collect();
    let big_race = Race {
        allotted_time: badly_kerned_race_time.parse().unwrap(),
        record_distance: badly_kerned_race_distance.parse().unwrap(),
    };
    let answer = ways_to_win_race(&big_race);
    println!("Part 2: {}", answer);
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let mut lines = contents.lines();
    let durations: Vec<_> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    let records: Vec<_> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    let races: Vec<Race> = (0..durations.len())
        .map(|idx| Race {
            allotted_time: durations[idx],
            record_distance: records[idx],
        })
        .collect();
    part_1(&races);
    part_2(&races);
}
