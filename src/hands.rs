use std::cmp::{Ord, Ordering, PartialOrd};
use cards::{Card, Value};

#[derive(Debug, Clone, Copy)]
struct Pokerhand {
    cards: [Card; 5],
}

impl Pokerhand {
    fn score(&self) -> HandScore {
        unimplemented!();
    }
}

pub enum HandScore {
    HighCard(Card),
    OnePair(Card),
    TwoPair(TwoPairScore),
    ThreeOfAKind(Card),
    Straight(Card, Card, Card, Card, Card),
    Flush(Card, Card, Card, Card, Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    StraightFlush(Card, Card, Card, Card, Card),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct TwoPairScore {
    high: Value,
    low: Value,
}

impl TwoPairScore {
    fn new(c1: Value, c2: Value) -> Result<Self, ()> {
        if c1 == c2 {
            return Err(());
        }
        if c1 > c2 {
            Ok(TwoPairScore { high: c1, low: c2 })
        } else {
            Ok(TwoPairScore { high: c2, low: c1 })
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct StraightScore {
    top: Value,
    hi: Value,
    mid: Value,
    low: Value,
    bot: Value,
    
}
impl StraightScore {
    fn new(c1: Value, c2: Value, c3: Value, c4: Value, c5: Value) -> Result<Self, ()> {
        let mut cards = [c1, c2, c3, c4, c5,];
        cards.sort();
    }
        
        
        

}
