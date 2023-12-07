use std::{cmp::min, collections::HashMap};

use regex::bytes::Regex;

fn main() {
    let input = include_str!("../input");
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let regex = Regex::new(r"\d+").unwrap();

    let mut part_1 = 0;
    let mut gear_ratios: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for m in regex.find_iter(line) {
            if let Some((x, y)) = surrounding_symbol(&lines, y, m.start(), m.end()) {
                let number = std::str::from_utf8(m.as_bytes())
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                part_1 += number;

                if lines[y][x] == b'*' {
                    gear_ratios.entry((x, y)).or_default().push(number);
                }
            }
        }
    }

    let part_2: usize = gear_ratios
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}

fn surrounding_symbol(
    lines: &Vec<&[u8]>,
    y: usize,
    start: usize,
    end: usize,
) -> Option<(usize, usize)> {
    let start = start.saturating_sub(1);
    let end = end + 1;

    let line = lines[y];

    for x in start..min(line.len(), end) {
        if y > 0 && is_symbol(lines[y - 1][x]) {
            return Some((x, y - 1));
        }
        if is_symbol(lines[y][x]) {
            return Some((x, y));
        }
        if y < lines.len() - 1 && is_symbol(lines[y + 1][x]) {
            return Some((x, y + 1));
        }
    }

    None
}

fn is_symbol(b: u8) -> bool {
    b != b'.' && !b.is_ascii_digit()
}
