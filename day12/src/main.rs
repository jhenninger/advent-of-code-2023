use std::collections::HashMap;

type Cache = HashMap<(usize, usize, u32), usize>;

#[derive(Default)]
struct Springs {
    springs: Vec<u8>,
    groups: Vec<u32>,
    cache: Cache,
}

impl Springs {
    fn new(s: &str) -> Self {
        let (springs, groups) = s.split_once(' ').unwrap();
        let groups: Vec<u32> = groups.split(',').map(|n| n.parse().unwrap()).collect();
        Self {
            springs: springs.as_bytes().to_vec(),
            groups,
            cache: HashMap::new(),
        }
    }

    fn arrangements(&mut self, si: usize, gi: usize, run: u32) -> usize {
        let key = (si, gi, run);

        if let Some(cached) = self.cache.get(&key) {
            return *cached;
        }

        let result = if si == self.springs.len() {
            (gi == self.groups.len() || (gi == self.groups.len() - 1 && self.groups[gi] == run))
                as usize
        } else {
            let mut sum = 0;
            let spring = self.springs[si];

            // broken or ?
            if spring != b'.' && gi < self.groups.len() && self.groups[gi] != run {
                sum += self.arrangements(si + 1, gi, run + 1)
            }

            // working or ?
            if spring != b'#' && (run == 0 || run == self.groups[gi]) {
                sum += self.arrangements(si + 1, if run == 0 { gi } else { gi + 1 }, 0);
            }

            sum
        };

        self.cache.insert(key, result);

        result
    }
}

fn main() {
    let input = include_str!("../input");
    let mut rows: Vec<_> = input.lines().map(Springs::new).collect();

    let part_1: usize = rows.iter_mut().map(|r| r.arrangements(0, 0, 0)).sum();
    let part_2: usize = rows
        .iter()
        .map(|r| {
            let mut s = Springs::default();
            for _ in 0..5 {
                s.springs.extend_from_slice(&r.springs);
                s.springs.push(b'?');
                s.groups.extend_from_slice(&r.groups);
            }

            s.springs.pop();

            s.arrangements(0, 0, 0)
        })
        .sum();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
