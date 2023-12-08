fn main() {
    // no parsing because I'm lazy

    let part_1: usize = [(53, 313), (89, 1090), (76, 1214), (98, 1201)]
        .iter()
        .map(|&(time, distance)| ways_to_win(time, distance))
        .product();

    let part_2 = ways_to_win(53_89_76_98, 313_1090_1214_1201);

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}

fn ways_to_win(time: u64, distance: u64) -> usize {
    (1..=time)
        .filter(|speed| (time - speed) * speed > distance)
        .count()
}
