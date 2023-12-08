use d7::hands::GameHand;
use std::fs;

#[derive(Debug)]
struct HandWithBid {
    hand: GameHand,
    bid: u64,
}

fn parse_input(input: &str) -> Vec<HandWithBid> {
    let mut hands_with_bid: Vec<_> = input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(" ").collect();
            let hand = GameHand::try_from(split[0]).unwrap();
            let bid = split[1].parse::<u64>().unwrap();
            HandWithBid { hand, bid }
        })
        .collect();
    hands_with_bid.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    hands_with_bid
}

fn part_1(hands_with_bids: &[HandWithBid]) {
    let mut total: u64 = 0;
    for (mult, hand) in hands_with_bids.iter().enumerate() {
        total += hand.bid * (mult + 1) as u64;
    }
    println!("Part 1: {}", total)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let hands_with_bids = parse_input(&input);
    part_1(&hands_with_bids);
}
