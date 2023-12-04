use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    let mut cards: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line[9..].split(" | ").map(|p| {
                p.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<HashSet<u32>>()
            });

            let wins = parts
                .next()
                .unwrap()
                .intersection(&parts.next().unwrap())
                .count();

            (1, wins)
        })
        .collect();

    let part_1: u32 = cards
        .iter()
        .filter(|(_, w)| *w > 0)
        .map(|(_, w)| 2u32.pow(*w as u32 - 1))
        .sum();

    for i in 0..cards.len() {
        let (count, wins) = cards[i];
        for j in 0..wins {
            cards[i + j + 1].0 += count;
        }
    }

    let part_2: u32 = cards.iter().map(|c| c.0).sum();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
