use super::card::CardError;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardValue {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eigth,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Distribution<CardValue> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardValue {
        match rng.gen_range(0..=13) {
            2 => CardValue::Two,
            3 => CardValue::Three,
            4 => CardValue::Four,
            5 => CardValue::Five,
            6 => CardValue::Six,
            7 => CardValue::Seven,
            8 => CardValue::Eigth,
            9 => CardValue::Nine,
            10 => CardValue::T,
            11 => CardValue::J,
            12 => CardValue::Q,
            13 => CardValue::K,
            _ => CardValue::A,
        }
    }
}

impl Display for CardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardValue::Two => write!(f, "2"),
            CardValue::Three => write!(f, "3"),
            CardValue::Four => write!(f, "4"),
            CardValue::Five => write!(f, "5"),
            CardValue::Six => write!(f, "6"),
            CardValue::Seven => write!(f, "7"),
            CardValue::Eigth => write!(f, "8"),
            CardValue::Nine => write!(f, "9"),
            CardValue::T => write!(f, "t"),
            CardValue::J => write!(f, "j"),
            CardValue::Q => write!(f, "q"),
            CardValue::K => write!(f, "k"),
            CardValue::A => write!(f, "a"),
        }
    }
}

impl FromStr for CardValue {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(CardValue::Two),
            "3" => Ok(CardValue::Three),
            "4" => Ok(CardValue::Four),
            "5" => Ok(CardValue::Five),
            "6" => Ok(CardValue::Six),
            "7" => Ok(CardValue::Seven),
            "8" => Ok(CardValue::Eigth),
            "9" => Ok(CardValue::Nine),
            "t" => Ok(CardValue::T),
            "j" => Ok(CardValue::J),
            "q" => Ok(CardValue::Q),
            "k" => Ok(CardValue::K),
            "a" => Ok(CardValue::A),
            _ => Err(CardError::UnknownCardValue),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::card::{card::CardError, cardvalue::CardValue};

    #[test]
    fn test_value_to_string() {
        let two = CardValue::Two;
        let three = CardValue::Three;
        let four = CardValue::Four;
        let five = CardValue::Five;
        let six = CardValue::Six;
        let seven = CardValue::Seven;
        let eight = CardValue::Eigth;
        let nine = CardValue::Nine;
        let ten = CardValue::T;
        let jack = CardValue::J;
        let queen = CardValue::Q;
        let king = CardValue::K;
        let ace = CardValue::A;

        assert_eq!(two.to_string(), "2");
        assert_eq!(three.to_string(), "3");
        assert_eq!(four.to_string(), "4");
        assert_eq!(five.to_string(), "5");
        assert_eq!(six.to_string(), "6");
        assert_eq!(seven.to_string(), "7");
        assert_eq!(eight.to_string(), "8");
        assert_eq!(nine.to_string(), "9");
        assert_eq!(ten.to_string(), "t");
        assert_eq!(jack.to_string(), "j");
        assert_eq!(queen.to_string(), "q");
        assert_eq!(king.to_string(), "k");
        assert_eq!(ace.to_string(), "a");
    }

    #[test]
    fn test_card_value_fromstr() {
        let card_value_ok = CardValue::from_str("3").unwrap();
        let card_value_err = CardValue::from_str("x").unwrap_err();

        assert_eq!(card_value_ok, CardValue::Three);
        assert_eq!(card_value_err, CardError::UnknownCardValue);
    }
}
