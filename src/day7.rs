use anyhow::{bail, Context, Error};
use core::num;
use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::Deref;
use std::{
    any,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, Sub},
    path::Path,
};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            _ => panic!("invalid char for Card"),
        }
    }
}

impl Card {
    fn value(&self) -> usize {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 1, // not it's joker, weakest but can act like other cards in Type creation
            Card::T => 10,
            Card::C9 => 9,
            Card::C8 => 8,
            Card::C7 => 7,
            Card::C6 => 6,
            Card::C5 => 5,
            Card::C4 => 4,
            Card::C3 => 3,
            Card::C2 => 2,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug)]
enum Type {
    FiveOfKind(Card),
    FourOfKind(Card),
    FullHouse(Card, Card),
    ThreeOfKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card),
}

impl Type {
    fn type_value(&self) -> usize {
        match self {
            Type::FiveOfKind(_) => 7,
            Type::FourOfKind(_) => 6,
            Type::FullHouse(_, _) => 5,
            Type::ThreeOfKind(_) => 4,
            Type::TwoPair(_, _) => 3,
            Type::OnePair(_) => 2,
            Type::HighCard(_) => 1,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.type_value().cmp(&other.type_value())
    }
}

impl From<[Card; 5]> for Type {
    fn from(value: [Card; 5]) -> Self {
        println!("type creation: {:?}", value);
        let mut return_first = false;
        if !value.contains(&Card::J) {
            return_first = true;
        }

        let mut varaints: HashSet<Card> = value.into_iter().filter(|c| c != &Card::J).collect();
        if varaints.is_empty() {
            // all J
            varaints.insert(Card::A);
        }

        let mut types = vec![];
        for variant in varaints {
            let mut local_value = vec![];
            for c in value {
                if c == Card::J {
                    local_value.push(variant);
                } else {
                    local_value.push(c);
                }
            }
            dbg!(&local_value);

            let mut m = HashMap::new();
            for v in local_value.clone() {
                m.entry(v).and_modify(|v| *v += 1).or_insert(1);
            }

            let typ = match m.len() {
                5 => Type::HighCard(local_value.into_iter().max().unwrap()),
                4 => Type::OnePair(
                    m.into_iter()
                        .filter(|(k, v)| *v == 2)
                        .map(|(k, _)| k)
                        .next()
                        .unwrap(),
                ),
                3 => {
                    // three of kind or two pair

                    if m.iter().any(|(_, v)| *v == 3) {
                        // three of kind
                        Type::ThreeOfKind(
                            m.into_iter()
                                .filter(|(k, v)| *v == 3)
                                .map(|(k, _)| k)
                                .next()
                                .unwrap(),
                        )
                    } else {
                        let mut cards = m.into_iter().filter(|(k, v)| *v == 2).map(|(k, _)| k);
                        Type::TwoPair(cards.next().unwrap(), cards.next().unwrap())
                    }
                }
                2 => {
                    // Full house or four of a kind

                    if m.iter().any(|(_, v)| *v == 3) {
                        // three of kind
                        let mut cards = m.into_iter();

                        let (c1, c2) = (cards.next().unwrap(), cards.next().unwrap());

                        match (c1, c2) {
                            ((card1, 3), (card2, 2)) => Type::FullHouse(card1, card2),
                            ((card1, 2), (card2, 3)) => Type::FullHouse(card2, card1),
                            _ => panic!("invalid pattern"),
                        }
                    } else {
                        Type::FourOfKind(
                            m.into_iter()
                                .filter(|(k, v)| *v == 4)
                                .map(|(k, _)| k)
                                .next()
                                .unwrap(),
                        )
                    }
                }
                1 => Type::FiveOfKind(m.into_iter().next().unwrap().0),
                _ => panic!("lol"),
            };
            if return_first {
                return typ;
            }
            types.push(typ);
        }

        dbg!(&types);
        types
            .into_iter()
            .max_by(|t1: &Type, t2: &Type| t1.cmp(&t2))
            .unwrap()
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    typ: Type,
}

impl Hand {
    fn new(hand: &str) -> Hand {
        assert_eq!(hand.len(), 5);

        let mut cards = vec![];
        for ch in hand.chars() {
            cards.push(Card::from(ch));
        }

        let cards_array: [Card; 5] = cards.try_into().unwrap();
        Hand {
            cards: cards_array,
            typ: Type::from(cards_array),
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match self.typ.cmp(&other.typ) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                println!("equal type: {:?} and {:?}", self, other);
                for i in 0..5 {
                    let card_cmp = self.cards[i].cmp(&other.cards[i]);
                    if card_cmp != Ordering::Equal {
                        println!("returning: {:?}", card_cmp);
                        return card_cmp;
                    }
                }
                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut res = 0;

    let mut hands: Vec<(Hand, usize)> = reader
        .lines()
        .map(|line| {
            let binding = line.unwrap();
            let (hand, bet) = binding.split_once(" ").unwrap();

            let h = Hand::new(hand);
            (h, bet.parse::<usize>().unwrap())
        })
        .collect();

    hands.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
    dbg!(&hands);
    let res: usize = hands
        .into_iter()
        .enumerate()
        .map(|(inx, (_, bet))| (inx + 1) * bet)
        .sum();

    println!("result day7 a: {res}",);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day7::{Card, Type};
    use std::cmp::Ordering;

    #[test]
    fn test_type_order() {
        let t1 = Type::FiveOfKind(Card::A);
        let t2 = Type::FiveOfKind(Card::Q);

        assert_eq!(t1.cmp(&t2), Ordering::Equal)
    }

    #[test]
    fn test_card_order() {
        assert_eq!(Card::A.cmp(&Card::K), Ordering::Greater)
    }
}
