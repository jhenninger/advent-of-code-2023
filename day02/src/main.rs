use std::cmp::max;

use regex::Regex;

#[derive(Default)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("../input");
    let regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let games: Vec<_> = input
        .lines()
        .map(|l| {
            let mut game = Game::default();

            for (_, [cubes, color]) in regex.captures_iter(l).map(|c| c.extract()) {
                let color = match color {
                    "red" => &mut game.red,
                    "green" => &mut game.green,
                    "blue" => &mut game.blue,
                    _ => panic!(),
                };
                *color = max(*color, cubes.parse().unwrap());
            }

            game
        })
        .collect();

    let part_1: usize = games
        .iter()
        .enumerate()
        .filter(|(_, g)| g.red <= 12 && g.green <= 13 && g.blue <= 14)
        .map(|(i, _)| i + 1)
        .sum();

    let part_2: u32 = games.iter().map(|g| g.red * g.blue * g.green).sum();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
