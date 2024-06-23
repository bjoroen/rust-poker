use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum HandRanking {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalStraightFlush,
}

impl Display for HandRanking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandRanking::HighCard => write!(f, "High card"),
            HandRanking::Pair => write!(f, "Pair"),
            HandRanking::TwoPair => write!(f, "Two pair"),
            HandRanking::ThreeOfAKind => write!(f, "Three of a kind"),
            HandRanking::Straight => write!(f, "Straight"),
            HandRanking::Flush => write!(f, "Flush"),
            HandRanking::FullHouse => write!(f, "Full house"),
            HandRanking::FourOfAKind => write!(f, "Four of a kind"),
            HandRanking::StraightFlush => write!(f, "Straight flush"),
            HandRanking::RoyalStraightFlush => write!(f, "Royal straight flush"),
        }
    }
}
