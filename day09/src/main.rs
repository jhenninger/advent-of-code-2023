fn main() {
    let input = include_str!("../input");

    let (part_1, part_2) = input.lines().fold((0, 0), |a, l| {
        let mut current: Vec<i32> = l
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let mut list = Vec::new();

        while current.iter().any(|e| *e != 0) {
            let next = current.windows(2).map(|w| w[1] - w[0]).collect();
            list.push(current);
            current = next;
        }

        let (part_1, part_2) = list.iter().rev().fold((0, 0), |a, l| {
            ((l.last().unwrap() + a.0), (l.first().unwrap() - a.1))
        });

        (a.0 + part_1, a.1 + part_2)
    });

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
