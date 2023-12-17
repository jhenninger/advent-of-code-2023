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

    let part_1 = dijkstra(&map, 0, 3);
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

    heap.push((Reverse(map[&(1, 0)]), (1, 0), (1, 0)));
    heap.push((Reverse(map[&(0, 1)]), (0, 1), (0, 1)));

    while let Some((Reverse(cost), pos, steps)) = heap.pop() {
        if pos == target {
            return cost;
        }

        let sx = steps.0.signum();
        let sy = steps.1.signum();

        // straight, turn left, turn right
        for next_step in [(steps.0 + sx, steps.1 + sy), (sy, sx), (-sy, -sx)] {
            if steps.0.abs() < min
                && steps.1.abs() < min
                && (next_step.0.abs() == 1 || next_step.1.abs() == 1)
            {
                // cant turn yet
                continue;
            }

            if next_step.0 > max || next_step.1 > max {
                // cant go straight
                continue;
            }

            let next_pos = (pos.0 + next_step.0.signum(), pos.1 + next_step.1.signum());

            if let Some(loss) = map.get(&next_pos) {
                let next_cost = cost + loss;
                if next_cost < *dist.get(&(next_pos, next_step)).unwrap_or(&u32::MAX) {
                    heap.push((Reverse(next_cost), next_pos, next_step));
                    dist.insert((next_pos, next_step), next_cost);
                }
            }
        }
    }

    unreachable!();
}