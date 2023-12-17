use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    let input = include_str!("../input");

    let map: HashMap<(i32, i32), u32> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap()))
        })
        .collect();

    let part_1 = dijkstra(&map, 1, 3);
    let part_2 = dijkstra(&map, 4, 10);

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}

fn dijkstra(map: &HashMap<(i32, i32), u32>, min: i32, max: i32) -> u32 {
    let target = (
        map.keys().map(|k| k.0).max().unwrap(),
        map.keys().map(|k| k.1).max().unwrap(),
    );

    let mut heap: BinaryHeap<(Reverse<u32>, (i32, i32), (i32, i32))> = BinaryHeap::new();
    let mut dist: HashMap<((i32, i32), (i32, i32)), u32> = HashMap::new();

    heap.push((Reverse(0), (0, 0), (0, 0)));

    while let Some((Reverse(cost), pos, dir)) = heap.pop() {
        if pos == target {
            return cost;
        }

        for next_dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if dir == next_dir || dir == (-next_dir.0, -next_dir.1) {
                continue;
            }

            let mut next_pos = pos;
            let mut next_cost = cost;

            for i in 1..=max {
                next_pos.0 += next_dir.0;
                next_pos.1 += next_dir.1;

                if let Some(loss) = map.get(&next_pos) {
                    next_cost += loss;
                    if i >= min && next_cost < *dist.get(&(next_pos, next_dir)).unwrap_or(&u32::MAX)
                    {
                        heap.push((Reverse(next_cost), next_pos, next_dir));
                        dist.insert((next_pos, next_dir), next_cost);
                    }
                }
            }
        }
    }

    unreachable!();
}
