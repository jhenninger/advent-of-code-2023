use std::collections::HashMap;

use num::integer::lcm;

struct Network<'a> {
    instructions: &'a str,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Network<'a> {
    fn new(s: &'a str) -> Self {
        let (instructions, map) = s.split_once("\n\n").unwrap();

        let map = map
            .lines()
            .map(|l| (&l[0..3], (&l[7..10], &l[12..15])))
            .collect();

        Self { instructions, map }
    }

    fn steps(&self, start: &str, goal: fn(&str) -> bool) -> usize {
        let mut key = start;

        for (i, c) in self.instructions.chars().cycle().enumerate() {
            if goal(key) {
                return i;
            }

            let pairs = self.map[key];

            key = match c {
                'L' => pairs.0,
                'R' => pairs.1,
                _ => panic!(),
            };
        }

        unreachable!();
    }
}

fn main() {
    let input = include_str!("../input");
    let network = Network::new(input);

    let part_1 = network.steps("AAA", |e| e == "ZZZ");

    let part_2 = network
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| network.steps(k, |e| e.ends_with('Z')))
        .reduce(lcm)
        .unwrap();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
