use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;
use std::usize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<&char> for Card {
    type Error = &'static str;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("Invalid card"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SpareCards<const N: usize> {
    pub cards: [Card; N],
}

impl<const N: usize> PartialOrd for SpareCards<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut self_cards: Vec<_> = self.cards.iter().collect();
        let mut other_cards: Vec<_> = other.cards.iter().collect();
        self_cards.sort_by(|a, b| b.cmp(a));
        other_cards.sort_by(|a, b| b.cmp(a));
        let mut self_iter = self_cards.iter();
        let mut other_iter = other_cards.iter();
        loop {
            let next_self = self_iter.next();
            let next_other = other_iter.next();
            match (next_self, next_other) {
                (Some(self_high), Some(other_high)) => match self_high.cmp(other_high) {
                    Ordering::Equal => continue,
                    neq => return Some(neq),
                },
                (Some(_), None) | (None, Some(_)) => panic!("Unequal N for SpareCards<N>"),
                _ => break,
            }
        }
        Some(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::Card::*;
    use super::SpareCards;
    use std::cmp::Ordering;

    #[test]
    fn test_card_ord() {
        assert!(Ace > King);
        assert!(King == King);
        assert!(Queen < King);
    }

    #[test]
    fn test_spare_cards_eq() {
        let some_cards = SpareCards {
            cards: [Jack, Seven, Queen],
        };
        let some_other_cards = SpareCards {
            cards: [Queen, Seven, Jack],
        };
        assert!(some_cards.partial_cmp(&some_other_cards) == Some(Ordering::Equal));
    }

    #[test]
    fn test_spare_cards_less() {
        let some_cards = SpareCards {
            cards: [Jack, Seven, Queen],
        };
        let some_other_cards = SpareCards {
            cards: [Queen, Seven, Ace],
        };
        assert!(some_cards.partial_cmp(&some_other_cards) == Some(Ordering::Less));
    }

    #[test]
    fn test_spare_cards_greater() {
        let some_cards = SpareCards {
            cards: [Ace, Queen, Eight],
        };
        let some_other_cards = SpareCards {
            cards: [Queen, Seven, Ace],
        };
        assert!(some_cards.partial_cmp(&some_other_cards) == Some(Ordering::Greater));
    }

    #[test]
    fn test_more_specific_spare_cards_less() {
        let first = SpareCards {
            cards: [Two, Three, Four, Five, Jack],
        };
        let second = SpareCards {
            cards: [Two, Three, Four, Five, Ace],
        };
        let third: SpareCards<5> = SpareCards {
            cards: [Jack, Three, Four, Five, Ace],
        };
        assert!((first < second) && (second < third));
    }
}
