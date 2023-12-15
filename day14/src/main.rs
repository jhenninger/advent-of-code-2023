use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Platform {
    rocks: Vec<Vec<u8>>,
}

impl Platform {
    fn new(s: &str) -> Self {
        let rocks = s.lines().map(|l| l.as_bytes().to_vec()).collect();
        Self { rocks }
    }

    fn load(&self) -> usize {
        self.rocks
            .iter()
            .enumerate()
            .map(|(y, l)| l.iter().filter(|c| **c == b'O').count() * (self.rocks.len() - y))
            .sum()
    }

    fn tilt_north(&mut self) {
        for x in 0..self.rocks[0].len() {
            let mut base = 0;
            for y in 0..self.rocks.len() {
                match self.rocks[y][x] {
                    b'O' => {
                        self.rocks[y][x] = b'.';
                        self.rocks[base][x] = b'O';
                        base += 1;
                    }
                    b'#' => base = y + 1,
                    _ => {}
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.rocks.len() {
            let mut base = 0;
            for x in 0..self.rocks[y].len() {
                match self.rocks[y][x] {
                    b'O' => {
                        self.rocks[y][x] = b'.';
                        self.rocks[y][base] = b'O';
                        base += 1;
                    }
                    b'#' => base = x + 1,
                    _ => {}
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.rocks[0].len() {
            let mut base = self.rocks.len() - 1;
            for y in (0..self.rocks.len()).rev() {
                match self.rocks[y][x] {
                    b'O' => {
                        self.rocks[y][x] = b'.';
                        self.rocks[base][x] = b'O';
                        base = base.saturating_sub(1);
                    }
                    b'#' => base = y.saturating_sub(1),
                    _ => {}
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.rocks.len() {
            let mut base = self.rocks[y].len() - 1;
            for x in (0..self.rocks[y].len()).rev() {
                match self.rocks[y][x] {
                    b'O' => {
                        self.rocks[y][x] = b'.';
                        self.rocks[y][base] = b'O';
                        base = base.saturating_sub(1);
                    }
                    b'#' => base = x.saturating_sub(1),
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let mut platform = Platform::new(input);

    platform.tilt_north();
    let part_1 = platform.load();

    let mut seen = HashMap::new();

    let cycles = 1_000_000_000;

    for i in 0..cycles {
        if let Some(prev) = seen.insert(platform.clone(), i) {
            if (cycles - i) % (i - prev) == 0 {
                break;
            }
        }

        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();
    }

    let part_2 = platform.load();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
