struct Almanac {
    maps: Vec<Vec<(u64, u64, u64)>>,
}

impl Almanac {
    fn seed_to_location(&self, value: u64) -> u64 {
        self.maps.iter().fold(value, |value, map| {
            map.iter()
                .find(|(_, src, len)| *src <= value && src + len >= value)
                .map(|(dst, src, _)| value - src + dst)
                .unwrap_or(value)
        })
    }

    fn location_to_seed(&self, value: u64) -> u64 {
        self.maps.iter().rev().fold(value, |value, map| {
            map.iter()
                .find(|(dst, _, len)| *dst <= value && dst + len >= value)
                .map(|(dst, src, _)| value + src - dst)
                .unwrap_or(value)
        })
    }
}

fn main() {
    let mut blocks = include_str!("../input").split("\n\n");

    let seeds: Vec<u64> = blocks
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let maps = blocks
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|l| {
                    let nums: Vec<_> = l
                        .split_ascii_whitespace()
                        .map(|i| i.parse::<u64>().unwrap())
                        .collect();
                    (nums[0], nums[1], nums[2])
                })
                .collect()
        })
        .collect();

    let almanac = Almanac { maps };

    let part_1 = seeds
        .iter()
        .map(|s| almanac.seed_to_location(*s))
        .min()
        .unwrap();

    let seed_ranges: Vec<_> = seeds.chunks(2).map(|c| c[0]..c[0] + c[1]).collect();

    let part_2 = (0..)
        .find(|l| {
            let s = almanac.location_to_seed(*l);
            seed_ranges.iter().any(|r| r.contains(&s))
        })
        .unwrap();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
