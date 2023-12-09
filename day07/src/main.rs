use std::cmp;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Type {
    fn from_cards(cards: &[u8]) -> Self {
        let mut sets = [0u8; 15];

        for &c in cards {
            sets[c as usize] += 1;
        }

        sets.sort_unstable_by_key(|r| cmp::Reverse(*r));

        match sets.as_slice() {
            [5, ..] => Type::FiveOfAKind,
            [4, ..] => Type::FourOfAKind,
            [3, 2, ..] => Type::FullHouse,
            [3, ..] => Type::ThreeOfAKind,
            [2, 2, ..] => Type::TwoPair,
            [2, ..] => Type::OnePair,
            _ => Type::HighCard,
        }
    }
}

#[derive(Debug)]
struct Hand {
    bid: u32,
    cards: Vec<u8>,
    cards_jokers: Vec<u8>,
    rank: Type,
    rank_jokers: Type,
}

impl Hand {
    fn new(cards: Vec<u8>, bid: u32) -> Hand {
        let rank = Type::from_cards(&cards);

        let cards_jokers: Vec<_> = cards
            .iter()
            .cloned()
            .map(|c| if c == 11 { 1 } else { c })
            .collect();
        let no_jokers: Vec<_> = cards_jokers.iter().cloned().filter(|c| *c != 1).collect();
        let rank_jokers = Type::from_cards(&no_jokers);

        let rank_jokers = match 5 - no_jokers.len() {
            4 | 5 => Type::FiveOfAKind,
            3 => match rank_jokers {
                Type::OnePair => Type::FiveOfAKind,
                _ => Type::FourOfAKind,
            },
            2 => match rank_jokers {
                Type::ThreeOfAKind => Type::FiveOfAKind,
                Type::OnePair => Type::FourOfAKind,
                _ => Type::ThreeOfAKind,
            },
            1 => match rank_jokers {
                Type::FourOfAKind => Type::FiveOfAKind,
                Type::ThreeOfAKind => Type::FourOfAKind,
                Type::TwoPair => Type::FullHouse,
                Type::OnePair => Type::ThreeOfAKind,
                _ => Type::OnePair,
            },
            _ => rank_jokers,
        };

        Self {
            bid,
            cards,
            rank,
            cards_jokers,
            rank_jokers,
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let mut cards: Vec<_> = input
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(" ").unwrap();

            let bid = bid.parse().unwrap();
            let cards = cards
                .chars()
                .map(|c| {
                    c.to_digit(10).unwrap_or_else(|| match c {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!(),
                    }) as u8
                })
                .collect();

            Hand::new(cards, bid)
        })
        .collect();

    cards.sort_unstable_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.cards.cmp(&b.cards)));
    let part_1 = winnings(&cards);

    cards.sort_unstable_by(|a, b| {
        a.rank_jokers
            .cmp(&b.rank_jokers)
            .then_with(|| a.cards_jokers.cmp(&b.cards_jokers))
    });
    let part_2 = winnings(&cards);

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}

fn winnings(cards: &[Hand]) -> usize {
    cards
        .iter()
        .enumerate()
        .map(|(i, c)| (i + 1) * c.bid as usize)
        .sum()
}
