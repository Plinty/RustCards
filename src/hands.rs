use cards::{Card, Value};

#[derive(Debug, Clone, Copy)]
pub struct Pokerhand {
    pub cards: [Card; 5],
}

impl Pokerhand {
    fn score(self) -> HandScore {
        match (self.is_straight(), self.is_flush()) {
            (true, true) => return HandScore::StraightFlush(StraightScore::new(self).unwrap()),
            (false, true) => return HandScore::Flush(FlushScore::new(self).unwrap()),
            (true, false) => return HandScore::Straight(StraightScore::new(self).unwrap()),
            (false, false) => {}
        }

        unimplemented!()
    }

    fn is_straight(&self) -> bool {
        let mut cards = self.cards;
        cards.sort();
        cards.windows(2)
            .map(|s| s[0].value as u8 - s[1].value as u8)
            .all(|v| v == 1)
    }

    fn is_flush(&self) -> bool {
        self.cards.windows(2).all(|p| p[0].suit == p[1].suit)
    }
}

pub enum HandScore {
    HighCard(Value),
    OnePair(Value),
    TwoPair(TwoPairScore),
    ThreeOfAKind(Value),
    Straight(StraightScore),
    Flush(FlushScore),
    FullHouse(FullHouseScore),
    FourOfAKind(Value),
    StraightFlush(StraightScore),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct TwoPairScore {
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
pub struct StraightScore {
    top: Value,
    hi: Value,
    mid: Value,
    low: Value,
    bot: Value,
}

impl StraightScore {
    fn new(hand: Pokerhand) -> Result<Self, ()> {
        let Pokerhand { mut cards } = hand;
        cards.sort();
        let is_straight = cards
            .windows(2)
            .map(|s| s[0].value as u8 - s[1].value as u8)
            .all(|v| v == 1);
        if is_straight {
            let bot = cards[0].value;
            let low = cards[1].value;
            let mid = cards[2].value;
            let hi = cards[3].value;
            let top = cards[4].value;
            Ok(StraightScore {
                top,
                hi,
                mid,
                low,
                bot,
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct FlushScore {
    top: Value,
    hi: Value,
    mid: Value,
    low: Value,
    bot: Value,
}

impl FlushScore {
    fn new(hand: Pokerhand) -> Result<Self, ()> {
        let Pokerhand { mut cards } = hand;
        cards.sort();
        let is_flush = cards.windows(2).all(|p| p[0].suit == p[1].suit);
        if is_flush {
            let bot = cards[0].value;
            let low = cards[1].value;
            let mid = cards[2].value;
            let hi = cards[3].value;
            let top = cards[4].value;
            Ok(FlushScore {
                top,
                hi,
                mid,
                low,
                bot,
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct FullHouseScore {
    triplet: Value,
    pair: Value,
}

impl FullHouseScore {
    fn new(hand: Pokerhand) -> Result<Self, ()> {
        let Pokerhand { mut cards } = hand;
        cards.sort();

        if is_three_of_a_kind(&cards[..3]) {
            if (cards[2].value != cards[3].value) && is_pair(&cards[3..]) {
                Ok(FullHouseScore {
                    triplet: cards[0].value,
                    pair: cards[4].value,
                })
            } else {
                Err(())
            }
        } else if is_pair(&cards[..2]) {
            if is_three_of_a_kind(&cards[2..]) {
                Ok(FullHouseScore {
                    triplet: cards[4].value,
                    pair: cards[0].value,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

fn is_three_of_a_kind(cards: &[Card]) -> bool {
    if cards.len() != 3 {
        return false;
    }
    (cards[0].value == cards[1].value) && (cards[1].value == cards[2].value)
}

fn is_pair(cards: &[Card]) -> bool {
    if cards.len() != 2 {
        return false;
    }
    (cards[0].value == cards[1].value)
}

fn same_value(cards: &[Card]) -> bool {
    cards.windows(2).all(|w| w[0].value == w[1].value)
}
