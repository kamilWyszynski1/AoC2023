use std::{
    any,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{bail, Context, Error};

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day8;
mod day9;

fn main() -> anyhow::Result<()> {
    // solvea("input1.txt")?;
    // solveb("input1.txt")
    // day2::solvea("input2.txt")
    // day2::solveb("input2.txt")
    // day3::solvea("input3.txt")
    // day3::solveb("input3.txt")
    // day4::solvea("input4.txt")
    // day4::solveb("input4.txt")
    // day5::solvea("input5.txt")
    // day5::solveb("input5.txt")
    // day6::solvea("input6.txt")
    // day6::solveb("input6.txt")
    // day7::solvea("input7.txt")
    // day7b::solveb("input7.txt")
    // day8::solveb("input8.txt")
    // day9::solveb("input9.txt")
    // day10::solvea("input10.txt")
    // day11::solveb("input11.txt")
    // day12::solveb("input12.txt")
    // day14::solveb("input14.txt")
    // day15::solveb("input15.txt")
    // day19::solveb("input19.txt")
    // day20::solvea("input20.txt")
    day21::solvea("input21.txt")
}

pub mod day7b {
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
        use std::cmp::Ordering;

        use crate::day7b::{Card, Type};

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
}

pub mod day7 {
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
                Card::J => 11,
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
            let mut m = HashMap::new();

            for v in value {
                m.entry(v).and_modify(|v| *v += 1).or_insert(1);
            }

            match m.len() {
                5 => Type::HighCard(value.into_iter().max().unwrap()),
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
            }
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

    pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
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
        use std::cmp::Ordering;

        use crate::day7::{Card, Type};

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
}

pub mod day6 {
    use anyhow::{bail, Context, Error};
    use core::num;
    use itertools::Itertools;
    use std::{
        any,
        collections::{HashMap, HashSet},
        fs::File,
        io::{BufRead, BufReader},
        ops::{Index, Sub},
        path::Path,
    };

    pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let mut res = 0;

        let mut lines = reader.lines();
        let binding = lines.next().unwrap()?.replace(" ", "");
        let times: Vec<usize> = binding
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|v| v.trim().parse::<usize>().unwrap())
            .collect_vec();

        let binding = lines.next().unwrap()?.replace(" ", "");
        let distances: Vec<usize> = binding
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|v| v.trim().parse::<usize>().unwrap())
            .collect();

        let mut ways = vec![];
        for (time, record) in times.into_iter().zip(distances.into_iter()) {
            let mut possibilities = 0;

            for i in (0..time).rev() {
                let d = (time - i) * i;
                if d > record {
                    possibilities += 1
                }
            }
            if possibilities != 0 {
                ways.push(possibilities);
            }
        }

        println!("result day6 b: {:?}", ways.into_iter().product::<usize>());
        Ok(())
    }

    pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let mut res = 0;

        let mut lines = reader.lines();
        let binding = lines.next().unwrap()?;
        let times: Vec<usize> = binding
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|v| v.trim().parse::<usize>().unwrap())
            .collect_vec();

        let binding = lines.next().unwrap()?;
        let distances: Vec<usize> = binding
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|v| v.trim().parse::<usize>().unwrap())
            .collect();

        let mut ways = vec![];
        for (time, record) in times.into_iter().zip(distances.into_iter()) {
            let mut possibilities = 0;

            for i in (0..time).rev() {
                let d = (time - i) * i;
                println!("i: {i}, d: {d}");
                if d > record {
                    possibilities += 1
                }
            }
            if possibilities != 0 {
                ways.push(possibilities);
            }
        }

        println!("result day6 a: {:?}", ways.into_iter().product::<usize>());
        Ok(())
    }
}

pub mod day5 {
    use anyhow::{bail, Context, Error};
    use core::num;
    use itertools::Itertools;
    use std::{
        any,
        collections::{HashMap, HashSet},
        fs::File,
        io::{BufRead, BufReader},
        ops::{Index, Sub},
        path::Path,
    };

    #[derive(Debug)]
    struct Range {
        from: usize,
        to: usize,
    }

    impl Range {
        /*
        -----------
            -----------

            -----------
        --------

         */
        fn contains(&self, another: &Self) -> bool {
            self.from <= another.from && another.from <= self.to
                || another.from <= self.from && self.from <= another.to
        }

        fn map_intersection(&mut self, source: &Self, dest: &Self) -> Option<Vec<Self>> {
            let start = self.from.max(source.from);
            let end = self.to.min(source.to);

            if start > end {
                return None;
            }

            let diff = dest.from as isize - source.from as isize;

            println!(
                "   seed: {:?}, source: {:?}, start: {}, end: {} diff: {}",
                self, source, start, end, diff
            );

            let mut ranges = vec![];
            if self.from < start {
                ranges.push(Range {
                    from: self.from,
                    to: start - 1,
                })
            }
            if self.to > end {
                ranges.push(Range {
                    from: end + 1,
                    to: self.to,
                });
            }

            self.from = (start as isize + diff) as usize;
            self.to = (end as isize + diff) as usize;

            Some(ranges)
        }
    }

    pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let mut res = 0;

        let mut lines = vec![];

        for line in reader.lines() {
            lines.push(line?)
        }

        let mut lines_iter = lines.into_iter();
        let mut seeds_pairs: Vec<usize> = lines_iter
            .next()
            .context("no next")?
            .split_once(": ")
            .context("split onect")?
            .1
            .trim()
            .split(" ")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();

        let mut seeds = vec![];
        for chunk in seeds_pairs.chunks(2) {
            assert!(chunk.len() == 2);
            let first = chunk[0];
            let range = chunk[1];

            seeds.push(Range {
                from: first,
                to: first + range - 1,
            })
        }
        dbg!(seeds.len());

        lines_iter.next();

        let mut mappings: Vec<Vec<String>> = vec![];
        let mut curr = vec![];
        for line in lines_iter {
            if line.is_empty() {
                mappings.push(curr.clone());
                curr.clear();
                continue;
            }
            curr.push(line);
        }
        mappings.push(curr);
        dbg!(&seeds);

        for mapping in mappings {
            println!("======= {} =========", mapping[0]);
            let mut new_ranges = vec![];
            'seed: for seed in seeds.iter_mut() {
                for line in &mapping {
                    if line.contains("to") {
                        continue;
                    }

                    let ranges: [usize; 3] = line
                        .split(" ")
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                        .try_into()
                        .unwrap();
                    assert!(ranges.len() == 3);

                    let dest = Range {
                        from: ranges[0],
                        to: ranges[0] + ranges[2] - 1,
                    };
                    let source = Range {
                        from: ranges[1],
                        to: ranges[1] + ranges[2] - 1,
                    };

                    match seed.map_intersection(&source, &dest) {
                        Some(mut nr) => {
                            println!("after seed: {:?}, new: {:?}", seed, &nr);
                            new_ranges.append(&mut nr);
                            continue 'seed;
                        }
                        None => {}
                    }
                }
            }
            seeds.append(&mut new_ranges);
        }

        dbg!(&seeds);
        println!(
            "result day5 a: {:?}",
            seeds.into_iter().map(|r| r.from).min()
        );
        Ok(())
    }

    pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let mut res = 0;

        let mut lines = vec![];

        for line in reader.lines() {
            lines.push(line?)
        }

        let mut lines_iter = lines.into_iter();
        let mut seeds: Vec<usize> = lines_iter
            .next()
            .context("no next")?
            .split_once(": ")
            .context("split onect")?
            .1
            .trim()
            .split(" ")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();
        lines_iter.next();

        let mut mappings: Vec<Vec<String>> = vec![];
        let mut curr = vec![];
        for line in lines_iter {
            if line.is_empty() {
                mappings.push(curr.clone());
                curr.clear();
                continue;
            }
            curr.push(line);
        }
        mappings.push(curr);

        for mapping in mappings {
            println!("mapping: {}", mapping[0]);
            'seed: for seed in seeds.iter_mut() {
                for line in &mapping {
                    if line.contains("to") {
                        continue;
                    }

                    let ranges: [usize; 3] = line
                        .split(" ")
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                        .try_into()
                        .unwrap();
                    assert!(ranges.len() == 3);

                    let dest = ranges[0];
                    let source = ranges[1];
                    let range = ranges[2];

                    if (source..source + range).contains(seed) {
                        println!("{} {} {} {}", *seed, dest, source, range);
                        *seed = dest + (*seed - source);
                        println!("  {}", *seed);
                        continue 'seed;
                    }
                }
            }
        }

        println!("result day5 a: {:?}", seeds.into_iter().min().unwrap());
        Ok(())
    }
}

pub mod day4 {
    use core::num;
    use std::{
        any,
        collections::{HashMap, HashSet},
        fs::File,
        io::{BufRead, BufReader},
        ops::{Index, Sub},
        path::Path,
    };

    use anyhow::{bail, Context, Error};

    pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let mut lines = vec![];
        for line in reader.lines() {
            lines.push(line?);
        }

        let mut copies: HashMap<usize, usize> = HashMap::new();
        for i in 1..=lines.len() {
            copies.insert(i, 1);
        }

        for (i, line) in lines.into_iter().enumerate() {
            let (_, rest) = line.split_once(": ").context("could not split")?;

            let (winning, ours) = rest.split_once("|").context("could not split numbers")?;

            let winning_numbers: Vec<isize> = winning
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.trim().parse::<isize>().unwrap())
                .collect();
            let ours_numbers: HashSet<isize> = ours
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.trim().parse::<isize>().unwrap())
                .collect();

            let a: isize = ours_numbers
                .into_iter()
                .map(|num| winning_numbers.contains(&num) as isize)
                .sum();

            let a = a as usize;
            let copy = copies.get(&(i + 1)).unwrap();
            for _ in 0..*copy {
                for j in i + 2..i + 2 + a {
                    copies.entry(j).and_modify(|v| *v += 1);
                }
            }
        }

        println!("result day4 b: {}", copies.values().sum::<usize>());

        Ok(())
    }

    pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let mut res = 0;

        for line in reader.lines() {
            let line = line?;

            let (_, rest) = line.split_once(": ").context("could not split")?;

            let (winning, ours) = rest.split_once("|").context("could not split numbers")?;

            let winning_numbers: Vec<isize> = winning
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.trim().parse::<isize>().unwrap())
                .collect();
            dbg!(ours);
            let ours_numbers: HashSet<isize> = ours
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.trim().parse::<isize>().unwrap())
                .collect();

            let a: isize = ours_numbers
                .into_iter()
                .map(|num| winning_numbers.contains(&num) as isize)
                .sum();
            dbg!(a);

            if a != 0 {
                res += 2_isize.pow(a as u32 - 1);
            }
        }

        println!("result day4 a: {res}");
        Ok(())
    }
}

pub mod day3 {
    use core::num;
    use std::{
        any,
        fs::File,
        io::{BufRead, BufReader},
        ops::{Index, Sub},
        path::Path,
    };

    use anyhow::{bail, Context, Error};

    pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            lines.push(line);
        }

        let mut numbers = Vec::new();

        for (line_inx, line) in lines.clone().into_iter().enumerate() {
            let mut curr = String::new();
            let mut first_inx = 0;
            let mut last_inx = 0;
            let mut subnums: Vec<_> = vec![];
            for (i, ch) in line.chars().enumerate() {
                if ch.is_numeric() {
                    if curr.is_empty() {
                        first_inx = i;
                        last_inx = i;
                    } else {
                        last_inx = i
                    }
                    curr.push(ch);
                    if i == line.len() - 1 {
                        subnums.push((curr.parse::<usize>().unwrap(), first_inx, last_inx));
                        curr.clear();
                        first_inx = 0;
                        last_inx = 0;
                    }
                } else {
                    if !curr.is_empty() {
                        subnums.push((curr.parse::<usize>().unwrap(), first_inx, last_inx));
                        curr.clear();
                        first_inx = 0;
                        last_inx = 0;
                    }
                }
            }
            numbers.push(subnums);
        }

        let mut sum = 0;

        for (line_inx, line) in lines.clone().into_iter().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch == '*' {
                    let mut adjs = vec![];

                    if let Some(inx) = line_inx.checked_sub(1) {
                        let ns = numbers.get(inx).unwrap();
                        dbg!(i, ns);
                        for (number, first, last) in ns {
                            if check_b(i, *first, *last) {
                                adjs.push(*number);
                            }
                        }
                    }

                    let local_numbers = numbers.get(line_inx).unwrap();
                    for (number, first, last) in local_numbers {
                        if check_b(i, *first, *last) {
                            adjs.push(*number);
                        }
                    }

                    if let Some(next_nums) = numbers.get(line_inx + 1) {
                        for (number, first, last) in next_nums {
                            if check_b(i, *first, *last) {
                                adjs.push(*number);
                            }
                        }
                    }

                    dbg!(&adjs);
                    if adjs.len() == 2 {
                        sum += adjs[0] * adjs[1];
                    }
                }
            }
        }

        println!("result day3 b: {}", sum);
        Ok(())
    }

    fn check_b(i: usize, first: usize, last: usize) -> bool {
        i.max(first).sub(i.min(first)) < 2 || last.max(i).sub(last.min(i)) < 2
    }

    pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            lines.push(line);
        }

        let mut sum = 0;

        for (line_inx, line) in lines.clone().into_iter().enumerate() {
            let mut curr = String::new();
            let mut first_inx = 0;
            let mut last_inx = 0;
            for (i, ch) in line.chars().enumerate() {
                if ch.is_numeric() {
                    if curr.is_empty() {
                        first_inx = i;
                        last_inx = i;
                    } else {
                        last_inx = i
                    }
                    curr.push(ch);

                    if i == line.len() - 1 {
                        if check(&lines, line_inx, first_inx, last_inx) {
                            match line.get(first_inx..=last_inx) {
                                Some(number_str) => {
                                    // let mut l: String = String::new();
                                    // let b1 = std::io::stdin().read_line(&mut l).unwrap();

                                    let n = number_str.parse::<usize>()?;
                                    dbg!(number_str, n, line_inx, first_inx, last_inx + 1);
                                    sum += n;
                                }
                                None => {}
                            }
                        }
                        curr.clear();
                        first_inx = 0;
                        last_inx = 0;
                    }

                    continue;
                } else {
                    if !curr.is_empty() && check(&lines, line_inx, first_inx, last_inx + 1) {
                        match line.get(first_inx..=last_inx) {
                            Some(number_str) => {
                                let mut l: String = String::new();
                                // let b1 = std::io::stdin().read_line(&mut l).unwrap();

                                let n = number_str.parse::<usize>()?;
                                dbg!(number_str, n, line_inx, first_inx, last_inx + 1);
                                sum += n;
                            }
                            None => {}
                        }
                    }
                    curr.clear();
                    first_inx = 0;
                    last_inx = 0;
                }
            }
        }

        println!("result day3 a: {}", sum);
        Ok(())
    }

    fn check(lines: &Vec<String>, line_inx: usize, first: usize, last: usize) -> bool {
        for i in line_inx.checked_sub(1).unwrap_or_default()..=line_inx + 1 {
            for j in first.checked_sub(1).unwrap_or_default()..=last {
                match lines.get(i).and_then(|line| {
                    line.chars()
                        .nth(j)
                        .and_then(|ch| Some(!ch.is_numeric() && ch != '.'))
                }) {
                    Some(v) => {
                        if v {
                            return true;
                        }
                    }
                    None => {}
                }
            }
        }
        return false;
    }
}

pub mod day2 {
    use anyhow::{bail, Context};
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        // only 12 red cubes, 13 green cubes, and 14 blue cubes

        let mut res = 0;

        'lines: for line in reader.lines() {
            let line = line?;

            let (game, rest) = line.split_once(": ").context("could not split")?;

            let game_number = {
                let (_, number) = game
                    .split_once("Game ")
                    .context("could not split at game")?;
                number.parse::<i32>()
            }?;

            let games = rest.split("; ");

            let mut possible = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

            for gamel in games {
                let cubes = gamel.split(", ");
                for cube in cubes {
                    let (cube_number, cube_value) =
                        cube.split_once(" ").context("could not split at cube")?;
                    let cube_number = cube_number.parse::<i32>()?;

                    match possible.get_mut(cube_value) {
                        Some(v) => {
                            if *v < cube_number {
                                continue 'lines;
                            }
                            *v -= cube_number
                        }
                        None => continue 'lines,
                    }
                }
            }
            res += game_number;
        }

        println!("result day2 a: {res}");
        Ok(())
    }

    pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        // only 12 red cubes, 13 green cubes, and 14 blue cubes

        let mut res = 0;

        'lines: for line in reader.lines() {
            let line = line?;

            let (game, rest) = line.split_once(": ").context("could not split")?;

            let game_number = {
                let (_, number) = game
                    .split_once("Game ")
                    .context("could not split at game")?;
                number.parse::<i32>()
            }?;

            let games = rest.split("; ");

            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            for gamel in games {
                let cubes = gamel.split(", ");
                for cube in cubes {
                    let (cube_number, cube_value) =
                        cube.split_once(" ").context("could not split at cube")?;
                    let cube_number = cube_number.parse::<i32>()?;

                    match cube_value {
                        "red" => red = cube_number.max(red),
                        "blue" => blue = cube_number.max(blue),
                        "green" => green = cube_number.max(green),
                        _ => bail!("invalid cube value"),
                    }
                }
            }
            res += red * blue * green;
        }

        println!("result day2 a: {res}");
        Ok(())
    }
}

mod day1 {
    use anyhow::{bail, Context};
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);
        let mut sum = 0;

        for line in reader.lines() {
            let line = line?;

            let mut first: Option<char> = None;
            let mut next: Option<char> = None;

            for char in line.chars() {
                if char.is_numeric() {
                    if first.is_none() {
                        first = Some(char)
                    } else {
                        next = Some(char)
                    }
                }
            }

            if next.is_none() {
                sum += format!("{}{}", first.unwrap(), first.unwrap()).parse::<i32>()?
            } else {
                sum += format!("{}{}", first.unwrap(), next.unwrap()).parse::<i32>()?
            }
        }
        println!("output a: {sum}");
        Ok(())
    }

    fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);
        let mut sum = 0;

        for line in reader.lines() {
            let mut line = line?;

            let first = take_first(line.chars().next().unwrap().to_string(), line.clone())?;
            let last = take_last(line.chars().last().unwrap().to_string(), line.clone())?;

            println!("{first}{last}");
            sum += format!("{first}{last}").parse::<i32>()?;
        }
        println!("output b: {sum}");
        Ok(())
    }

    fn take_first(curr: String, line: String) -> anyhow::Result<i32> {
        let ch = curr.chars().last().context("no first char")?;

        if ch.is_numeric() {
            return Ok(ch.to_string().parse::<i32>()?);
        } else {
            if curr.ends_with("one") {
                return Ok(1);
            }
            if curr.ends_with("two") {
                return Ok(2);
            }
            if curr.ends_with("three") {
                return Ok(3);
            }
            if curr.ends_with("four") {
                return Ok(4);
            }
            if curr.ends_with("five") {
                return Ok(5);
            }
            if curr.ends_with("six") {
                return Ok(6);
            }
            if curr.ends_with("seven") {
                return Ok(7);
            }
            if curr.ends_with("eight") {
                return Ok(8);
            }
            if curr.ends_with("nine") {
                return Ok(9);
            }
        }
        take_first(line[0..curr.len() + 1].to_string(), line)
    }

    fn take_last(curr: String, line: String) -> anyhow::Result<i32> {
        let ch = curr.chars().next().context("no first char")?;

        if ch.is_numeric() {
            return Ok(ch.to_string().parse::<i32>()?);
        } else {
            if curr.starts_with("one") {
                return Ok(1);
            }
            if curr.starts_with("two") {
                return Ok(2);
            }
            if curr.starts_with("three") {
                return Ok(3);
            }
            if curr.starts_with("four") {
                return Ok(4);
            }
            if curr.starts_with("five") {
                return Ok(5);
            }
            if curr.starts_with("six") {
                return Ok(6);
            }
            if curr.starts_with("seven") {
                return Ok(7);
            }
            if curr.starts_with("eight") {
                return Ok(8);
            }
            if curr.starts_with("nine") {
                return Ok(9);
            }
        }
        if curr.len() == line.len() {
            bail!("empty");
        }
        take_last(line[line.len() - curr.len() - 1..].to_string(), line)
    }
}
