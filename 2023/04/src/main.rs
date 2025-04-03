use std::{io, collections::HashSet};


#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    mine: Vec<u32>,
}


impl Card {
    fn parse(text: &str) -> Card {
        let start_i = text.find(':').unwrap() + 1;
        let mut parts = text[start_i..].split('|');

        let winning = parts.next().unwrap()
            .split_whitespace()
            .map(|str| str.parse().unwrap())
            .collect();
        let mine = parts.next().unwrap()
            .split_whitespace()
            .map(|str| str.parse().unwrap())
            .collect();

        Card {
            winning,
            mine,
        }
    }


    fn get_count(&self) -> u32 {
        let winning: HashSet<_> = self.winning.iter().collect();
        let mine: HashSet<_> = self.mine.iter().collect();
        let count = winning.intersection(&mine).count();

        count as u32
    }
}


fn main() {
    let cards: Vec<_> = io::stdin().lines()
        .map(|line| Card::parse(&line.unwrap()))
        .collect();

    let res = solve1(&cards);
    println!("{}", res);

    let res = solve2(&cards);
    println!("{}", res);
}


fn solve1(cards: &[Card]) -> u32 {
    let mut res = 0;

    for card in cards {
        let count = card.get_count();
        res += get_points(count);
    }

    res
}


fn solve2(cards: &[Card]) -> u32 {
    let mut counts = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let count = card.get_count() as usize;
        for j in 0..count {
            counts[i + j + 1] += counts[i];
        }
    }

    counts.iter().sum()
}


fn get_points(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }

    2u32.pow(count - 1)
}
