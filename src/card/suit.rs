use super::card::CardError;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

impl Distribution<Suit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        match rng.gen_range(0..=3) {
            1 => Suit::Heart,
            2 => Suit::Spade,
            3 => Suit::Diamond,
            _ => Suit::Club,
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Heart => write!(f, "h"),
            Suit::Spade => write!(f, "s"),
            Suit::Diamond => write!(f, "r"),
            Suit::Club => write!(f, "k"),
        }
    }
}

impl FromStr for Suit {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "h" => Ok(Suit::Heart),
            "s" => Ok(Suit::Spade),
            "r" => Ok(Suit::Diamond),
            "k" => Ok(Suit::Club),
            _ => Err(CardError::UnknownSuit),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::card::{card::CardError, suit::Suit};

    #[test]
    fn test_suit_to_string() {
        let heart = Suit::Heart;
        let spade = Suit::Spade;
        let diamond = Suit::Diamond;
        let club = Suit::Club;

        assert_eq!(heart.to_string(), "h");
        assert_eq!(spade.to_string(), "s");
        assert_eq!(diamond.to_string(), "r");
        assert_eq!(club.to_string(), "k")
    }

    #[test]
    fn test_suit_fromstr() {
        let suit_ok = Suit::from_str("r").unwrap();
        let suit_err = Suit::from_str("x").unwrap_err();

        assert_eq!(suit_ok, Suit::Diamond);
        assert_eq!(suit_err, CardError::UnknownSuit);
    }
}
