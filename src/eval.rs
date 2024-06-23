use std::{collections::HashMap, fmt::Display};

use crate::{
    card::{card::Card, cardvalue::CardValue, suit::Suit},
    hand_ranks::HandRanking,
};

#[derive(Debug, PartialEq, Eq)]
pub enum HandError {
    NotEnoughCards,
    TooManyCards,
    DuplicateCards,
}

impl Display for HandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandError::NotEnoughCards => write!(f, "Not enough cards"),
            HandError::TooManyCards => write!(f, "Too many cards"),
            HandError::DuplicateCards => write!(f, "Duplicate cards"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Eval {
    hand: Vec<Card>,
}

impl Eval {
    pub fn from(cards: Vec<Card>) -> Eval {
        Eval { hand: cards }
    }

    pub fn evaluate(&self) -> Result<HandRanking, HandError> {
        if let Err(hand_error) = self.validate_hand() {
            return Err(hand_error);
        }

        let mut card_values: Vec<CardValue> = self.hand.iter().map(|card| card.value).collect();
        let mut suits: Vec<Suit> = self.hand.iter().map(|card| card.suit).collect();

        card_values.sort();
        suits.sort();

        let mut rank_count = HashMap::new();
        for &rank in &card_values {
            *rank_count.entry(rank).or_insert(0) += 1;
        }

        let counts: Vec<i32> = rank_count.values().cloned().collect();

        let rankings = self.max_rank(&card_values, &suits, &counts);

        if let Some(max_ranking) = rankings.iter().max() {
            Ok(max_ranking.clone())
        } else {
            Ok(HandRanking::HighCard)
        }
    }

    fn validate_hand(&self) -> Result<(), HandError> {
        if self.hand.len() > 5 {
            return Err(HandError::TooManyCards);
        }

        if self.hand.len() < 5 {
            return Err(HandError::NotEnoughCards);
        }

        let mut hand = self.hand.clone();
        hand.sort();
        hand.dedup();

        if hand.len() != 5 {
            return Err(HandError::DuplicateCards);
        }

        Ok(())
    }

    fn max_rank(
        &self,
        card_values: &Vec<CardValue>,
        suits: &Vec<Suit>,
        counts: &Vec<i32>,
    ) -> Vec<HandRanking> {
        let mut rankings: Vec<HandRanking> = Vec::new();

        // Pair
        // Pair is a hand that contains two cards of one rank and three cards of three other ranks
        // Example: ["kh", "qh", "5s", "3r", "kr"]
        if is_pair(&counts) {
            rankings.push(HandRanking::Pair);
        }
        // Two pair
        // Two pair is a hand that contains two cards of one rank, two cards of another rank and one card of a third rank
        // Example: ["kh", "qh", "qs", "3r", "kr"]
        if is_two_pair(&counts) {
            rankings.push(HandRanking::TwoPair);
        }
        // Three of a kind
        // Three of a kind is a hand that contains three cards of one rank and two cards of two other ranks
        // Example: ["kh", "ks", "qs", "3r", "kr"]
        if is_three_of_a_kind(&counts) {
            rankings.push(HandRanking::ThreeOfAKind);
        }
        // Straight
        // A straight is a hand that contains five cards of sequential rank
        // Example: ["ah", "2s", "3k", "4r", "5r"]
        if is_straight(&card_values) {
            rankings.push(HandRanking::Straight)
        };
        // Flush
        // A flush is a hand that contains five cards all of the same suit
        // Example: ["ah", "7h", "qh", "th", "2h"]
        if is_flush(&suits) {
            rankings.push(HandRanking::Flush)
        }
        // Full House
        // Full house is a hand that contains three cards of one rank and two cards of another rank
        // Example: ["ah", "ar", "qh", "qr", "qs"]
        if is_pair(&counts) && is_three_of_a_kind(&counts) {
            rankings.push(HandRanking::FullHouse)
        }
        // Four of kind
        // Four of a kind, also known as quads, is a hand that contains four cards of one rank and one card of another rank
        // Example: ["ah", "ar", "as", "ak", "qs"]
        if is_four_of_a_kind(&counts) {
            rankings.push(HandRanking::FourOfAKind);
        }
        // Straight flush
        // A straight flush is a hand that contains five cards of sequential rank, all of the same suit
        // Example: ["ah", "2h", "3h", "4h", "5h"]
        if is_straight(&card_values) && is_flush(&suits) {
            rankings.push(HandRanking::StraightFlush);
        }
        // Royal straight flush
        // Royal straight flush is a hand with an ace-high straight flush from 10 to ace
        // Example: ["th", "jh", "qh", "kh", "ah"]
        if is_royal_straight_flush(&card_values, &suits) {
            rankings.push(HandRanking::RoyalStraightFlush)
        }

        rankings
    }
}

fn is_pair(count: &Vec<i32>) -> bool {
    count.contains(&2)
}

fn is_three_of_a_kind(counts: &Vec<i32>) -> bool {
    counts.contains(&3)
}

fn is_four_of_a_kind(counts: &Vec<i32>) -> bool {
    counts.contains(&4)
}

fn is_two_pair(counts: &Vec<i32>) -> bool {
    counts.iter().filter(|&&count| count == 2).count() == 2
}

fn is_straight(card_ranks: &Vec<CardValue>) -> bool {
    // Since `card_ranks` is sorted, we can check that the next card is greater than the previous card.
    // If this is true for all elements, it's a straight.
    //
    // We also check if `card_ranks` is equal to [2,3,4,5,a] to handle the edge case where ace can be 1
    card_ranks
        .windows(2)
        .all(|c| c[0] as usize + 1 == c[1] as usize)
        || *card_ranks
            == vec![
                CardValue::Two,
                CardValue::Three,
                CardValue::Four,
                CardValue::Five,
                CardValue::A,
            ]
}

fn is_flush(suits: &Vec<Suit>) -> bool {
    suits.windows(2).all(|s| s[0] == s[1])
}

fn is_royal_straight_flush(card_ranks: &Vec<CardValue>, suits: &Vec<Suit>) -> bool {
    // We check if it is a straight, and the value of the cards sum up to 50(10 + 11 + 12 + 13 + 14)
    // which means it has to be a royal straight
    let is_royal_straight = is_straight(&card_ranks)
        && card_ranks.into_iter().map(|cr| *cr as usize).sum::<usize>() == 60;

    is_royal_straight && is_flush(&suits)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_is_too_many_cards_error() {
        let cards: Vec<Card> = ["kr", "js", "7s", "ts", "3h", "2h"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap_err();

        assert_eq!(evaluation, HandError::TooManyCards)
    }

    #[test]
    fn test_is_not_enough_cards_error() {
        let cards: Vec<Card> = ["kr", "js", "7s", "ts"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap_err();

        assert_eq!(evaluation, HandError::NotEnoughCards)
    }

    #[test]
    fn test_is_duplicate_cards_error() {
        let cards: Vec<Card> = ["kr", "js", "7s", "ts", "ts"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap_err();

        assert_eq!(evaluation, HandError::DuplicateCards)
    }

    #[test]
    fn test_is_high_card() {
        let cards: Vec<Card> = ["kr", "js", "7s", "ts", "3h"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap();

        assert_eq!(evaluation, HandRanking::HighCard)
    }

    #[test]
    fn test_is_pair() {
        let cards: Vec<Card> = ["kr", "ks", "7s", "ts", "3h"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap();

        assert_eq!(evaluation, HandRanking::Pair)
    }

    #[test]
    fn test_is_two_pair() {
        let cards: Vec<Card> = ["kr", "ks", "jh", "js", "3h"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap();

        assert_eq!(evaluation, HandRanking::TwoPair)
    }

    #[test]
    fn test_is_three_of_a_kind() {
        let cards: Vec<Card> = ["kr", "ks", "kh", "ts", "3h"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap();

        assert_eq!(evaluation, HandRanking::ThreeOfAKind)
    }

    #[test]
    fn test_is_straight() {
        let cards: Vec<Card> = ["ts", "jh", "qh", "kr", "ar"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation_one = eval.evaluate().unwrap();

        let cards_two: Vec<Card> = ["as", "5h", "3h", "4r", "2r"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval_two = Eval::from(cards_two);
        let evaluation_two = eval_two.evaluate().unwrap();

        assert_eq!(evaluation_one, HandRanking::Straight);
        assert_eq!(evaluation_two, HandRanking::Straight)
    }

    #[test]
    fn test_is_flush() {
        let cards: Vec<Card> = ["ts", "js", "9s", "2s", "5s"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap();

        assert_eq!(evaluation, HandRanking::Flush)
    }

    #[test]
    fn test_is_full_house() {
        let cards: Vec<Card> = ["ts", "tr", "9s", "9r", "9h"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap();

        assert_eq!(evaluation, HandRanking::FullHouse)
    }

    #[test]
    fn test_is_four_of_a_kind() {
        let cards: Vec<Card> = ["kr", "ks", "kh", "kk", "3h"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap();

        assert_eq!(evaluation, HandRanking::FourOfAKind)
    }

    #[test]
    fn test_is_straight_flush() {
        let cards: Vec<Card> = ["4s", "2s", "5s", "3s", "as"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap();

        assert_eq!(evaluation, HandRanking::StraightFlush)
    }

    #[test]
    fn test_is_royal_straight_flush() {
        let cards: Vec<Card> = ["ts", "js", "qs", "ks", "as"]
            .map(|c| Card::from_str(c).unwrap())
            .to_vec();

        let eval = Eval::from(cards);
        let evaluation = eval.evaluate().unwrap();

        assert_eq!(evaluation, HandRanking::RoyalStraightFlush)
    }
}
