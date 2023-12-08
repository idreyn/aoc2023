use crate::cards::{Card, SpareCards};
use std::{
    cmp::{max, min, Ordering, PartialEq, PartialOrd},
    collections::HashMap,
};

#[derive(Clone, Debug)]
pub struct GenericHand {
    pub cards: [Card; 5],
    counts: HashMap<Card, usize>,
}

impl GenericHand {
    pub fn new(cards: &[Card; 5]) -> Self {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        for card in cards {
            counts
                .entry(card.to_owned())
                .and_modify(|a| *a += 1)
                .or_insert(1);
        }
        Self {
            cards: cards.to_owned(),
            counts: counts,
        }
    }

    pub fn find_all_n_of(&self, n: usize) -> Vec<&Card> {
        self.counts
            .iter()
            .filter_map(|entry| match entry {
                (card, count) => {
                    if *count == n {
                        Some(card)
                    } else {
                        None
                    }
                }
            })
            .collect()
    }

    pub fn find_n_of(&self, n: usize) -> Option<&Card> {
        self.find_all_n_of(n).first().copied()
    }
}

impl TryFrom<&str> for GenericHand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chars: Vec<_> = value.chars().collect();
        if chars.len() != 5 {
            return Err(());
        }
        let mut card_vec = Vec::new();
        for char in chars {
            let card = Card::try_from(&char);
            if card.is_err() {
                return Err(());
            }
            card_vec.push(card.unwrap());
        }
        Ok(Self::new(&[
            card_vec[0],
            card_vec[1],
            card_vec[2],
            card_vec[3],
            card_vec[4],
        ]))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FiveOfAKind {
    kind: Card,
}

impl PartialOrd for FiveOfAKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.kind.partial_cmp(&other.kind)
    }
}

impl TryFrom<&GenericHand> for FiveOfAKind {
    type Error = ();

    fn try_from(hand: &GenericHand) -> Result<Self, Self::Error> {
        if hand.counts.len() == 1 {
            Ok(Self {
                kind: hand.cards[0].to_owned(),
            })
        } else {
            Err(())
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FourOfAKind {
    kind: Card,
    spare: SpareCards<1>,
}

impl PartialOrd for FourOfAKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(Ordering::Equal) => self.spare.partial_cmp(&other.spare),
            neq => neq,
        }
    }
}

impl TryFrom<&GenericHand> for FourOfAKind {
    type Error = ();

    fn try_from(hand: &GenericHand) -> Result<Self, Self::Error> {
        let four_of = hand.find_n_of(4);
        match four_of {
            Some(kind) => {
                let spare = hand.find_n_of(1).unwrap();
                Ok(Self {
                    kind: kind.to_owned(),
                    spare: SpareCards {
                        cards: [spare.to_owned()],
                    },
                })
            }
            None => Err(()),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FullHouse {
    high_kind: Card,
    low_kind: Card,
}

impl PartialOrd for FullHouse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.high_kind.partial_cmp(&other.high_kind) {
            Some(Ordering::Equal) => self.low_kind.partial_cmp(&other.low_kind),
            _ => self.high_kind.partial_cmp(&other.high_kind),
        }
    }
}

impl TryFrom<&GenericHand> for FullHouse {
    type Error = ();

    fn try_from(hand: &GenericHand) -> Result<Self, Self::Error> {
        let three_of = hand.find_n_of(3);
        let two_of = hand.find_n_of(2);
        match (three_of, two_of) {
            (Some(three_of), Some(two_of)) => Ok(Self {
                high_kind: three_of.to_owned(),
                low_kind: two_of.to_owned(),
            }),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ThreeOfAKind {
    kind: Card,
    spares: SpareCards<2>,
}

impl PartialOrd for ThreeOfAKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(Ordering::Equal) => self.spares.partial_cmp(&other.spares),
            neq => neq,
        }
    }
}

impl TryFrom<&GenericHand> for ThreeOfAKind {
    type Error = ();

    fn try_from(value: &GenericHand) -> Result<Self, Self::Error> {
        let three_of = value.find_n_of(3);
        let remaining = value.find_all_n_of(1);
        if let Some(kind) = three_of {
            if remaining.len() == 2 {
                return Ok(Self {
                    kind: kind.to_owned(),
                    spares: SpareCards {
                        cards: [remaining[0].to_owned(), remaining[1].to_owned()],
                    },
                });
            }
        }
        Err(())
    }
}

#[derive(PartialEq, Clone, Debug)]

pub struct TwoPair {
    high_kind: Card,
    low_kind: Card,
    spares: SpareCards<1>,
}

impl PartialOrd for TwoPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.high_kind.partial_cmp(&other.high_kind) {
            Some(Ordering::Equal) => match self.low_kind.partial_cmp(&other.low_kind) {
                Some(Ordering::Equal) => self.spares.partial_cmp(&other.spares),
                neq => neq,
            },
            neq => neq,
        }
    }
}

impl TryFrom<&GenericHand> for TwoPair {
    type Error = ();

    fn try_from(value: &GenericHand) -> Result<Self, Self::Error> {
        let two_of = value.find_all_n_of(2);
        let remaining = value.find_n_of(1);
        if two_of.len() == 2 {
            let higher_pair = max(two_of[0], two_of[1]);
            let lower_pair = min(two_of[0], two_of[1]);
            return Ok(Self {
                high_kind: higher_pair.to_owned(),
                low_kind: lower_pair.to_owned(),
                spares: SpareCards {
                    cards: [remaining.unwrap().to_owned()],
                },
            });
        }
        Err(())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Pair {
    kind: Card,
    spares: SpareCards<3>,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(Ordering::Equal) => self.spares.partial_cmp(&other.spares),
            neq => neq,
        }
    }
}

impl TryFrom<&GenericHand> for Pair {
    type Error = ();

    fn try_from(value: &GenericHand) -> Result<Self, Self::Error> {
        let two_of = value.find_n_of(2);
        let remaining = value.find_all_n_of(1);
        if let Some(kind) = two_of {
            if remaining.len() == 3 {
                return Ok(Self {
                    kind: kind.to_owned(),
                    spares: SpareCards {
                        cards: [
                            remaining[0].to_owned(),
                            remaining[1].to_owned(),
                            remaining[2].to_owned(),
                        ],
                    },
                });
            }
        }
        Err(())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct HighCard {
    spares: SpareCards<5>,
}

impl PartialOrd for HighCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.spares.partial_cmp(&other.spares)
    }
}

impl TryFrom<&GenericHand> for HighCard {
    type Error = ();

    fn try_from(value: &GenericHand) -> Result<Self, Self::Error> {
        let remaining = value.find_all_n_of(1);
        if remaining.len() == 5 {
            return Ok(Self {
                spares: SpareCards {
                    cards: [
                        remaining[0].to_owned(),
                        remaining[1].to_owned(),
                        remaining[2].to_owned(),
                        remaining[3].to_owned(),
                        remaining[4].to_owned(),
                    ],
                },
            });
        }
        Err(())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum GameHandKind {
    HighCard(HighCard),
    Pair(Pair),
    TwoPair(TwoPair),
    ThreeOfAKind(ThreeOfAKind),
    FullHouse(FullHouse),
    FourOfAKind(FourOfAKind),
    FiveOfAKind(FiveOfAKind),
}

#[derive(PartialEq, Clone, Debug)]
pub struct GameHand {
    kind: GameHandKind,
    cards: [Card; 5],
}

impl GameHand {
    fn hand_order(&self) -> usize {
        match self.kind {
            GameHandKind::HighCard(_) => 0,
            GameHandKind::Pair(_) => 1,
            GameHandKind::TwoPair(_) => 2,
            GameHandKind::ThreeOfAKind(_) => 3,
            GameHandKind::FullHouse(_) => 4,
            GameHandKind::FourOfAKind(_) => 5,
            GameHandKind::FiveOfAKind(_) => 6,
        }
    }

    fn cmp_with_other_by_cards(&self, other: &GameHand) -> Ordering {
        let self_cards: Vec<_> = self.cards.iter().collect();
        let other_cards: Vec<_> = other.cards.iter().collect();
        let mut self_iter = self_cards.iter();
        let mut other_iter = other_cards.iter();
        loop {
            match (self_iter.next(), other_iter.next()) {
                (Some(self_next), Some(other_next)) => match self_next.cmp(other_next) {
                    Ordering::Equal => continue,
                    neq => return neq,
                },
                (None, None) => return Ordering::Equal,
                _ => unreachable!("Invalid hand comparison"),
            }
        }
    }
}

impl TryFrom<&GenericHand> for GameHand {
    type Error = ();

    fn try_from(unknown: &GenericHand) -> Result<Self, Self::Error> {
        let five_of_kind = FiveOfAKind::try_from(unknown);
        if five_of_kind.is_ok() {
            let kind = GameHandKind::FiveOfAKind(five_of_kind.unwrap());
            return Ok(Self {
                kind,
                cards: unknown.cards,
            });
        }
        let four_of_kind = FourOfAKind::try_from(unknown);
        if four_of_kind.is_ok() {
            let kind = GameHandKind::FourOfAKind(four_of_kind.unwrap());
            return Ok(Self {
                kind,
                cards: unknown.cards,
            });
        }
        let full_house = FullHouse::try_from(unknown);
        if full_house.is_ok() {
            let kind = GameHandKind::FullHouse(full_house.unwrap());
            return Ok(Self {
                kind,
                cards: unknown.cards,
            });
        }
        let three_of_kind = ThreeOfAKind::try_from(unknown);
        if three_of_kind.is_ok() {
            let kind = GameHandKind::ThreeOfAKind(three_of_kind.unwrap());
            return Ok(Self {
                kind,
                cards: unknown.cards,
            });
        }
        let two_pair = TwoPair::try_from(unknown);
        if two_pair.is_ok() {
            let kind = GameHandKind::TwoPair(two_pair.unwrap());
            return Ok(Self {
                kind,
                cards: unknown.cards,
            });
        }
        let pair = Pair::try_from(unknown);
        if pair.is_ok() {
            let kind = GameHandKind::Pair(pair.unwrap());
            return Ok(Self {
                kind,
                cards: unknown.cards,
            });
        }
        let high_card = HighCard::try_from(unknown);
        if high_card.is_ok() {
            let kind = GameHandKind::HighCard(high_card.unwrap());
            return Ok(Self {
                kind,
                cards: unknown.cards,
            });
        }
        Err(())
    }
}

impl TryFrom<&str> for GameHand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let unknown = GenericHand::try_from(value);
        if unknown.is_err() {
            return Err(());
        }
        Self::try_from(&unknown.unwrap())
    }
}

impl PartialOrd for GameHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_order().partial_cmp(&other.hand_order()) {
            Some(Ordering::Equal) => Some(self.cmp_with_other_by_cards(&other)),
            neq => neq,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Card;

    #[test]
    fn test_try_five_of_kind_ok() {
        let hand = GenericHand::new(&[Card::Seven; 5]);
        let five_of_kind = FiveOfAKind::try_from(&hand);
        assert!(five_of_kind.is_ok());
    }

    #[test]
    fn test_try_five_of_kind_fails() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Eight,
            Card::Seven,
            Card::Seven,
        ]);
        let five_of_kind = FiveOfAKind::try_from(&hand);
        assert!(five_of_kind.is_err());
    }

    #[test]
    fn test_try_four_of_kind_ok() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Eight,
            Card::Seven,
            Card::Seven,
        ]);
        let four_of_kind = FourOfAKind::try_from(&hand);
        assert!(four_of_kind.is_ok());
    }

    #[test]
    fn test_try_four_of_kind_fails_on_five() {
        let hand = GenericHand::new(&[Card::Seven; 5]);
        let four_of_kind = FourOfAKind::try_from(&hand);
        assert!(four_of_kind.is_err());
    }

    #[test]
    fn test_try_four_of_kind_fails_on_three() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::King,
            Card::Eight,
        ]);
        let four_of_kind = FourOfAKind::try_from(&hand);
        assert!(four_of_kind.is_err());
    }

    #[test]
    fn test_try_full_house_ok() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::King,
            Card::King,
        ]);
        let full_house = FullHouse::try_from(&hand);
        assert!(full_house.is_ok());
    }

    #[test]
    fn test_full_house_fails_on_three_of_kind() {
        let hand =
            GenericHand::new(&[Card::Seven, Card::Seven, Card::Seven, Card::Ace, Card::King]);
        let full_house = FullHouse::try_from(&hand);
        assert!(full_house.is_err());
    }

    #[test]
    fn test_full_house_fails_on_two_pair() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Eight,
            Card::Eight,
            Card::King,
        ]);
        let full_house = FullHouse::try_from(&hand);
        assert!(full_house.is_err());
    }

    #[test]
    fn test_try_three_of_kind_ok() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::King,
            Card::Eight,
        ]);
        let three_of_kind = ThreeOfAKind::try_from(&hand);
        assert!(three_of_kind.is_ok());
    }

    #[test]
    fn test_try_three_of_kind_fails_on_full_house() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::King,
            Card::King,
        ]);
        let three_of_kind = ThreeOfAKind::try_from(&hand);
        assert!(three_of_kind.is_err());
    }

    #[test]
    fn test_try_three_of_kind_fails_on_pair() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::King,
            Card::Nine,
            Card::Eight,
        ]);
        let three_of_kind = ThreeOfAKind::try_from(&hand);
        assert!(three_of_kind.is_err());
    }

    #[test]
    fn test_try_two_pair_ok() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::King,
            Card::King,
            Card::Eight,
        ]);
        let two_pair = TwoPair::try_from(&hand);
        assert!(two_pair.is_ok());
        let the_two_pair = two_pair.unwrap();
        assert!(the_two_pair.high_kind == Card::King);
        assert!(the_two_pair.low_kind == Card::Seven);
    }

    #[test]
    fn test_try_two_pair_fails_on_one_pair() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Queen,
            Card::King,
            Card::Eight,
        ]);
        let two_pair = TwoPair::try_from(&hand);
        assert!(two_pair.is_err());
    }

    #[test]
    fn test_try_pair_ok() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Queen,
            Card::King,
            Card::Eight,
        ]);
        let pair = Pair::try_from(&hand);
        assert!(pair.is_ok());
        let the_pair = pair.unwrap();
        assert!(the_pair.kind == Card::Seven);
    }

    #[test]
    fn test_try_pair_fails_on_two_pair() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Queen,
            Card::Queen,
            Card::Eight,
        ]);
        let pair = Pair::try_from(&hand);
        assert!(pair.is_err());
    }

    #[test]
    fn test_try_pair_fails_on_three_of_kind() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::King,
            Card::Eight,
        ]);
        let pair = Pair::try_from(&hand);
        assert!(pair.is_err());
    }

    #[test]
    fn test_try_high_card_ok() {
        let hand =
            GenericHand::new(&[Card::Seven, Card::Ace, Card::Queen, Card::King, Card::Eight]);
        let high_card = HighCard::try_from(&hand);
        assert!(high_card.is_ok());
    }

    #[test]
    fn test_try_high_card_fails_on_pair() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Queen,
            Card::King,
            Card::Eight,
        ]);
        let high_card = HighCard::try_from(&hand);
        assert!(high_card.is_err());
    }

    #[test]
    fn test_try_high_card_fails_on_two_pair() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Queen,
            Card::Queen,
            Card::Eight,
        ]);
        let high_card = HighCard::try_from(&hand);
        assert!(high_card.is_err());
    }

    #[test]
    fn test_try_parse_full_house() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::King,
            Card::King,
        ]);
        let game_hand = GameHand::try_from(&hand);
        assert!(game_hand.is_ok());
        let the_game_hand = game_hand.unwrap();
        assert!(matches!(the_game_hand.kind, GameHandKind::FullHouse(_)));
    }

    #[test]
    fn test_try_parse_four_of_kind() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::King,
        ]);
        let game_hand = GameHand::try_from(&hand);
        assert!(game_hand.is_ok());
        let the_game_hand = game_hand.unwrap();
        assert!(matches!(the_game_hand.kind, GameHandKind::FourOfAKind(_)));
    }

    #[test]
    fn test_try_parse_five_of_kind() {
        let hand = GenericHand::new(&[Card::Seven; 5]);
        let game_hand = GameHand::try_from(&hand);
        assert!(game_hand.is_ok());
        let the_game_hand = game_hand.unwrap();
        assert!(matches!(the_game_hand.kind, GameHandKind::FiveOfAKind(_)));
    }

    #[test]
    fn test_cmp_two_hands() {
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::King,
        ]);
        let game_hand = GameHand::try_from(&hand);
        assert!(game_hand.is_ok());
        let the_game_hand = game_hand.unwrap();
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Ace,
        ]);
        let game_hand = GameHand::try_from(&hand);
        assert!(game_hand.is_ok());
        let other_game_hand = game_hand.unwrap();
        assert!(the_game_hand < other_game_hand);
    }

    #[test]
    fn test_cmp_two_different_hands() {
        let hand = GenericHand::new(&[Card::Seven, Card::Ace, Card::Ace, Card::King, Card::King]);
        let game_hand = GameHand::try_from(&hand);
        assert!(game_hand.is_ok());
        let the_game_hand = game_hand.unwrap();
        let hand = GenericHand::new(&[
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::King,
        ]);
        let game_hand = GameHand::try_from(&hand);
        assert!(game_hand.is_ok());
        let other_game_hand = game_hand.unwrap();
        assert!(the_game_hand < other_game_hand);
    }

    #[test]
    fn test_parse_hand_str() {
        let game_hand = GameHand::try_from("7AAAA");
        assert!(game_hand.is_ok());
        let the_game_hand = game_hand.unwrap();
        assert!(matches!(the_game_hand.kind, GameHandKind::FourOfAKind(_)));
    }

    #[test]
    fn test_parse_another_hand_str() {
        let game_hand = GameHand::try_from("7788A");
        assert!(game_hand.is_ok());
        let the_game_hand = game_hand.unwrap();
        assert!(matches!(the_game_hand.kind, GameHandKind::TwoPair(_)));
    }

    #[test]
    fn test_cmp_several_hands() {
        let first = GameHand::try_from("2345J").unwrap();
        let second = GameHand::try_from("2345A").unwrap();
        let third = GameHand::try_from("J345A").unwrap();
        assert!(first < second);
        assert!(second < third);
        assert!((first < second) && (second < third));
    }
}
