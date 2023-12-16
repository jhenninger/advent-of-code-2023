use std::cmp::min;

struct Mirror<'a>(Vec<&'a [u8]>);

impl<'a> Mirror<'a> {
    fn new(s: &'a str) -> Self {
        Self(s.lines().map(|l| l.as_bytes()).collect())
    }

    fn reflection(&self, smudges: usize) -> usize {
        'x: for x in 0..self.0[0].len() - 1 {
            let mut badness = 0;
            let to_check = min(x + 1, self.0[0].len() - x - 1);

            for dx in 0..to_check {
                for y in 0..self.0.len() {
                    if self.0[y][x - dx] != self.0[y][x + dx + 1] {
                        badness += 1;

                        if badness > smudges {
                            continue 'x;
                        }
                    }
                }
            }

            if badness == smudges {
                return x + 1;
            }
        }

        'y: for y in 0..self.0.len() - 1 {
            let mut badness = 0;
            let to_check = min(y + 1, self.0.len() - y - 1);

            for dy in 0..to_check {
                for x in 0..self.0[0].len() {
                    if self.0[y - dy][x] != self.0[y + dy + 1][x] {
                        badness += 1;

                        if badness > smudges {
                            continue 'y;
                        }
                    }
                }
            }

            if badness == smudges {
                return (y + 1) * 100;
            }
        }

        panic!();
    }
}

fn main() {
    let input = include_str!("../input");

    let mirrors: Vec<_> = input.split("\n\n").map(Mirror::new).collect();

    let part_1: usize = mirrors.iter().map(|m| m.reflection(0)).sum();
    let part_2: usize = mirrors.iter().map(|m| m.reflection(1)).sum();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
