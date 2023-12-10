use std::collections::{HashMap, HashSet};

type Point = (i32, i32);
type Map = HashMap<Point, char>;

fn main() {
    let input = include_str!("../input");

    let map: Map = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect();

    let start = *map.iter().find(|e| *e.1 == 'S').unwrap().0;
    let path = path(&map, start);

    println!(
        "Part 1: {}\nPart 2: {}",
        path.len() / 2,
        enclosed(&map, &path)
    );
}

fn path(map: &Map, start: Point) -> Vec<Point> {
    let mut path = vec![start];
    let mut last_pos = start;
    let mut current = [
        ((start.0 + 1, start.1), "-7J"),
        ((start.0 - 1, start.1), "-FL"),
        ((start.0, start.1 + 1), "|LJ"),
        ((start.0, start.1 - 1), "|F7"),
    ]
    .iter()
    .find(|(pos, s)| s.contains(map[&pos]))
    .unwrap()
    .0;

    while current != start {
        let dx = last_pos.0 - current.0;
        let dy = last_pos.1 - current.1;

        let (x, y) = current;
        path.push(current);
        last_pos = current;

        current = match (dx, dy, map[&current]) {
            (1, _, '-') => (x - 1, y),
            (-1, _, '-') => (x + 1, y),

            (_, 1, '|') => (x, y - 1),
            (_, -1, '|') => (x, y + 1),

            (-1, _, 'J') => (x, y - 1),
            (_, -1, 'J') => (x - 1, y),

            (1, _, 'L') => (x, y - 1),
            (_, -1, 'L') => (x + 1, y),

            (-1, _, '7') => (x, y + 1),
            (_, 1, '7') => (x - 1, y),

            (1, _, 'F') => (x, y + 1),
            (_, 1, 'F') => (x + 1, y),

            unknown => panic!("{:?}", unknown),
        };
    }

    path
}

fn enclosed(map: &Map, path: &[Point]) -> usize {
    let max_x = map.keys().map(|k| k.0).max().unwrap();
    let max_y = map.keys().map(|k| k.1).max().unwrap();

    let mut filled: HashSet<Point> = path.iter().cloned().collect();

    for w in path.windows(2) {
        let ((x1, y1), (x2, y2)) = (w[0], w[1]);

        let dx = x1 - x2;
        let dy = y1 - y2;

        // stay on left side of path
        let left = match (dx, dy) {
            (-1, 0) => (x2, y2 - 1),
            (1, 0) => (x2, y2 + 1),
            (0, -1) => (x2 + 1, y2),
            (0, 1) => (x2 - 1, y2),

            unknown => panic!("{:?}", unknown),
        };

        // in case of right turns we also have to fill the diagonal (one back and one left of current tile)
        // if it is not a right turn we already filled this tile, no harm done in checking it again
        let diagonal = (left.0 + dx, left.1 + dy);

        if fill(&mut filled, left, max_x, max_y) || fill(&mut filled, diagonal, max_x, max_y) {
            // we walked off the edge of the map, therefor we are filling the wrong side of the path
            let reverse_path: Vec<_> = path.iter().rev().cloned().collect();
            return enclosed(map, &reverse_path);
        }
    }

    filled.len() - path.len()
}

// returns true if we walk off the map
fn fill(filled: &mut HashSet<Point>, start: Point, max_x: i32, max_y: i32) -> bool {
    if start.0 < 0 || start.1 < 0 || start.0 > max_x || start.1 > max_y {
        return true;
    }

    if !filled.insert(start) {
        return false;
    }

    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (nx, ny) = (start.0 + dx, start.1 + dy);

        if fill(filled, (nx, ny), max_x, max_y) {
            return true;
        }
    }

    false
}
