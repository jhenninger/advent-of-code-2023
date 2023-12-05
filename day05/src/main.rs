use std::ops::RangeInclusive;

struct Map {
    ranges: Vec<(RangeInclusive<u64>, u64)>,
}

impl Map {
    fn convert(&self, value: u64) -> u64 {
        for (range, offset) in &self.ranges {
            if range.contains(&value) {
                return value - range.start() + offset;
            }
        }

        value
    }
}
fn main() {
    let mut input = include_str!("../input").split("\n\n");

    let seeds: Vec<u64> = input
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let maps: Vec<_> = input
        .map(|block| {
            let ranges = block
                .lines()
                .skip(1)
                .map(|l| {
                    let mut nums = l
                        .split_ascii_whitespace()
                        .map(|i| i.parse::<u64>().unwrap());
                    let dst = nums.next().unwrap();
                    let src = nums.next().unwrap();
                    let len = nums.next().unwrap();
                    (src..=src + len, dst)
                })
                .collect();

            Map { ranges }
        })
        .collect();

    let part_1 = seeds
        .iter()
        .cloned()
        .map(|s| maps.iter().fold(s, |s, m| m.convert(s)))
        .min()
        .unwrap();

    // BRUTE FORCE BABY
    let part_2 = seeds
        .chunks(2)
        .flat_map(|c| c[0]..c[0] + c[1])
        .map(|s| maps.iter().fold(s, |s, m| m.convert(s)))
        .min()
        .unwrap();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
