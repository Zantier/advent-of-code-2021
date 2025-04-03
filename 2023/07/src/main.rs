use std::{
    collections::HashMap,
    io,
};


const JOKER: i32 = 1;


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: Vec<i32>,
    bid: i32,
}


impl Hand {
    fn parse(line: &str) -> Hand {
        let mut split = line.split_whitespace();
        let cards: Vec<_> = split.next().unwrap()
            .chars()
            .map(|ch| parse_card(ch))
            .collect();
        let bid = split.next().unwrap().parse().unwrap();
        Hand { hand_type: Self::get_type(&cards), cards, bid }
    }


    fn add_jokers(&mut self) {
        for card in self.cards.iter_mut() {
            if *card == 11 {
                *card = JOKER;
            }
        }

        self.hand_type = Self::get_type(&self.cards);
    }


    fn get_type(cards: &[i32]) -> HandType {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for &card in cards {
            *counter.entry(card).or_default() += 1;
        }

        let mut values: Vec<_> = counter.values().cloned().collect();
        values.sort();
        values.reverse();

        let jokers = *counter.entry(JOKER).or_default();
        if jokers == values[0] {
            values[0] += values.get(1).cloned().unwrap_or_default();
        } else {
            values[0] += jokers;
        }

        if values[0] >= 5 {
            return HandType::Five;
        }
        if values[0] >= 4 {
            return HandType::Four;
        }
        if values[0] >= 3 && values[1] >= 2 {
            return HandType::Full;
        }
        if values[0] >= 3 {
            return HandType::Three;
        }
        if values[0] >= 2 && values[1] >= 2 {
            return HandType::TwoPair;
        }
        if values[0] >= 2 {
            return HandType::Pair;
        }

        return HandType::High;
    }
}


fn main() {
    let mut hands: Vec<_> = io::stdin().lines()
        .map(|line| Hand::parse(&line.unwrap()))
        .collect();
    hands.sort();

    let res = solve(&hands);
    println!("{}", res);

    for hand in hands.iter_mut() {
        hand.add_jokers();
    }
    hands.sort();
    let res = solve(&hands);
    println!("{}", res);
}


fn solve(hands: &[Hand]) -> i32 {
    let mut res = 0;
    for (i, hand) in hands.iter().enumerate() {
        res += (i as i32 + 1) * hand.bid;
    }

    res
}


fn parse_card(ch: char) -> i32 {
    match ch {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        ch => {
            match ch.to_digit(10) {
                Some(x) => x as i32,
                None => 0,
            }
        }
    }
}
