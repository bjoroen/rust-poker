use std::collections::HashSet;
use std::{fmt, str::FromStr};

use serde::Serialize;
use serde_with::DeserializeFromStr;

use super::cardvalue::CardValue;
use super::suit::Suit;

#[derive(Debug, PartialEq, Eq)]
pub enum CardError {
    UnknownCardValue,
    UnknownSuit,
    InvalidCard,
}

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardError::UnknownCardValue => write!(f, "Unknown card value"),
            CardError::UnknownSuit => write!(f, "Unknown suit"),
            CardError::InvalidCard => write!(f, "Invalid card"),
        }
    }
}

#[derive(
    Serialize, DeserializeFromStr, Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd,
)]
pub struct Card {
    pub value: CardValue,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

impl FromStr for Card {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(CardError::InvalidCard);
        }

        let as_char: Vec<char> = s.chars().collect();
        let value = CardValue::from_str(&String::from(as_char[0]))?;
        let suit = Suit::from_str(&String::from(as_char[1]))?;

        Ok(Self { value, suit })
    }
}

impl Card {
    pub fn new_hand() -> Vec<Card> {
        let mut cards = HashSet::new();
        while cards.len() != 5 {
            cards.insert(Card {
                value: rand::random(),
                suit: rand::random(),
            });
        }

        cards.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {

    use crate::card::suit::Suit;

    use super::*;

    #[test]
    fn test_new_hand_no_duplicates() {
        for _ in 0..100_00 {
            let mut hand: Vec<Card> = Card::new_hand();
            hand.sort();
            hand.dedup();

            assert_eq!(hand.len(), 5)
        }
    }

    #[test]
    fn test_card_to_string() {
        let card_one = Card {
            value: CardValue::K,
            suit: Suit::Spade,
        };

        let card_two = Card {
            value: CardValue::Two,
            suit: Suit::Diamond,
        };

        assert_eq!(card_one.to_string(), "ks");
        assert_eq!(card_two.to_string(), "2r");
    }

    #[test]
    fn test_card_fromstr() {
        let card_ok = Card::from_str("kh").unwrap();
        let card_value_err = Card::from_str("xh").unwrap_err();
        let card_suit_err = Card::from_str("kx").unwrap_err();
        let card_invalid_err = Card::from_str("khh").unwrap_err();

        assert_eq!(
            card_ok,
            Card {
                value: CardValue::K,
                suit: Suit::Heart
            }
        );
        assert_eq!(card_value_err, CardError::UnknownCardValue);
        assert_eq!(card_suit_err, CardError::UnknownSuit);
        assert_eq!(card_invalid_err, CardError::InvalidCard);
    }
}
