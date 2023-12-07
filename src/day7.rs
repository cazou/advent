use crate::traits::AdventOfCode;
use anyhow::Result;
use std::cmp::Ordering;

const CARD_ORDER: [[char; 13]; 2] = [
    [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ],
    [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ],
];

#[derive(Eq, PartialEq, Debug)]
struct Card {
    card: char,
    run: usize,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.card_id().cmp(&other.card_id())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    pub fn card_id(&self) -> usize {
        CARD_ORDER[self.run]
            .iter()
            .enumerate()
            .find_map(|(idx, c)| if *c == self.card { Some(idx) } else { None })
            .unwrap()
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bet: usize,
    run: usize,
    hand_type: HandType,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn from_str(s: &str, run: usize) -> Result<Hand> {
        let (cards_str, bet_str) = s.split_once(" ").unwrap();
        let cards = cards_str
            .chars()
            .map(|card| Card { card, run })
            .collect::<Vec<Card>>();
        let bet = bet_str.parse()?;
        let hand_type = Self::hand_type(&cards, run);

        Ok(Hand {
            cards,
            bet,
            run,
            hand_type,
        })
    }

    fn hand_type_cards(counts: &[u8; 13]) -> HandType {
        if counts.iter().filter(|i| **i == 5).count() == 1 {
            HandType::FiveOfAKind
        } else if counts.iter().filter(|i| **i == 4).count() == 1 {
            HandType::FourOfAKind
        } else if counts.iter().filter(|i| **i == 3).count() == 1
            && counts.iter().filter(|i| **i == 2).count() == 1
        {
            HandType::FullHouse
        } else if counts.iter().filter(|i| **i == 3).count() == 1 {
            HandType::ThreeOfAKind
        } else if counts.iter().filter(|i| **i == 2).count() == 2 {
            HandType::TwoPair
        } else if counts.iter().filter(|i| **i == 2).count() == 1 {
            HandType::OnePair
        } else if counts.iter().filter(|i| **i == 1).count() == 5 {
            HandType::HighCard
        } else {
            HandType::None
        }
    }

    fn hand_type(cards: &[Card], run: usize) -> HandType {
        let mut counts: [u8; 13] = [0; 13];
        for c in cards {
            counts[c.card_id()] += 1;
        }

        if run == 0 {
            return Self::hand_type_cards(&counts);
        }

        let jokers = counts[0];
        if jokers == 5 {
            return HandType::FiveOfAKind;
        }
        if jokers == 0 {
            return Self::hand_type_cards(&counts);
        }

        counts[0] = 0;
        let mut max = HandType::None;
        for c in 1..13 {
            if counts[c] > 0 {
                counts[c] += jokers;
                max = std::cmp::max(max, Self::hand_type_cards(&counts));
                counts[c] -= jokers;
            }
        }

        max
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            for i in 0..4 {
                if self.cards[i] != other.cards[i] {
                    return self.cards[i].cmp(&other.cards[i]);
                }
            }
            self.cards.last().unwrap().cmp(&other.cards.last().unwrap())
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day7;

impl AdventOfCode for Day7 {
    fn day(&self) -> u8 {
        7
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut hands: Vec<Hand> = vec![];
        for line in input.unwrap().lines() {
            hands.push(Hand::from_str(line, 0).unwrap());
        }

        hands.sort();

        let ret = hands
            .iter()
            .enumerate()
            .fold(0, |a, (rank, hand)| a + hand.bet * (rank + 1));

        Ok(ret.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut hands: Vec<Hand> = vec![];
        for line in input.unwrap().lines() {
            hands.push(Hand::from_str(line, 1).unwrap());
        }

        hands.sort();

        let ret = hands
            .iter()
            .enumerate()
            .fold(0, |a, (rank, hand)| a + hand.bet * (rank + 1));

        Ok(ret.to_string())
    }
}
