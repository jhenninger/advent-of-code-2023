use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

type Point = (i32, i32);

struct Grid(HashMap<Point, u8>);

impl Grid {
    fn new(s: &str) -> Self {
        Self(
            s.lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.as_bytes()
                        .iter()
                        .enumerate()
                        .map(move |(x, c)| ((x as i32, y as i32), *c))
                })
                .collect(),
        )
    }

    fn energized(&self, pos: Point, dir: Point) -> usize {
        let mut seen: HashSet<(Point, Point)> = HashSet::new();
        self.beam(&mut seen, pos, dir);
        seen.iter().map(|t| t.0).collect::<HashSet<_>>().len()
    }

    fn beam(&self, seen: &mut HashSet<(Point, Point)>, pos: Point, dir: Point) {
        if !self.0.contains_key(&pos) || !seen.insert((pos, dir)) {
            return;
        }

        match (self.0[&pos], dir) {
            (b'\\', _) => self.beam(seen, (pos.0 + dir.1, pos.1 + dir.0), (dir.1, dir.0)),
            (b'/', _) => self.beam(seen, (pos.0 - dir.1, pos.1 - dir.0), (-dir.1, -dir.0)),
            (b'-', (0, _)) => {
                self.beam(seen, (pos.0 + 1, pos.1), (1, 0));
                self.beam(seen, (pos.0 - 1, pos.1), (-1, 0));
            }
            (b'|', (_, 0)) => {
                self.beam(seen, (pos.0, pos.1 + 1), (0, 1));
                self.beam(seen, (pos.0, pos.1 - 1), (0, -1));
            }
            _ => self.beam(seen, (pos.0 + dir.0, pos.1 + dir.1), dir),
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let grid = Grid::new(input);
    let part_1 = grid.energized((0, 0), (1, 0));

    let width = grid.0.keys().map(|k| k.0).max().unwrap();
    let height = grid.0.keys().map(|k| k.1).max().unwrap();

    let mut part_2 = 0;

    for w in 0..=width {
        part_2 = max(grid.energized((w, 0), (0, 1)), part_2);
        part_2 = max(grid.energized((w, height), (0, -1)), part_2);
    }

    for h in 0..=height {
        part_2 = max(grid.energized((0, h), (1, 0)), part_2);
        part_2 = max(grid.energized((width, h), (-1, 0)), part_2);
    }

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
