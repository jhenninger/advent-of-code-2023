fn main() {
    let input = include_str!("../input");

    let universe: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x as i64, y as i64)),
                _ => None,
            })
        })
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        expand(universe.clone(), 2),
        expand(universe, 1000000)
    );
}

fn expand(mut universe: Vec<(i64, i64)>, factor: i64) -> i64 {
    let max_x = universe.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = universe.iter().map(|(_, y)| *y).max().unwrap();

    let empty_x: Vec<i64> = (1..max_x)
        .filter(|x| universe.iter().all(|g| g.0 != *x))
        .collect();
    let empty_y: Vec<i64> = (1..max_y)
        .filter(|y| universe.iter().all(|g| g.1 != *y))
        .collect();

    for x in empty_x.iter().rev() {
        for g in universe.iter_mut() {
            if g.0 > *x {
                g.0 += factor - 1;
            }
        }
    }

    for y in empty_y.iter().rev() {
        for g in universe.iter_mut() {
            if g.1 > *y {
                g.1 += factor - 1;
            }
        }
    }

    let mut distance = 0;

    for (i, (x1, y1)) in universe.iter().enumerate() {
        for (x2, y2) in universe.iter().skip(i + 1) {
            distance += (x1 - x2).abs() + (y1 - y2).abs();
        }
    }

    distance
}
