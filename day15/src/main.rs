fn main() {
    let input = include_str!("../input").trim();
    let instructions: Vec<_> = input.split(',').collect();

    let part_1: usize = instructions.iter().map(|instr| hash(instr) as usize).sum();

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for instr in &instructions {
        let (label, focal_len) = instr.split_once(['=', '-']).unwrap();
        let b = &mut boxes[hash(label) as usize];

        match focal_len.parse() {
            Ok(focal_len) => match b.iter_mut().find(|e| e.0 == label) {
                Some(lense) => lense.1 = focal_len,
                _ => b.push((label, focal_len)),
            },
            _ => b.retain(|e| e.0 != label),
        }
    }

    let part_2: usize = boxes
        .iter()
        .enumerate()
        .flat_map(|(bi, b)| {
            b.iter()
                .enumerate()
                .map(move |(li, l)| (bi + 1) * (li + 1) * l.1)
        })
        .sum();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}

fn hash(s: &str) -> u8 {
    s.as_bytes()
        .iter()
        .fold(0, |a, b| a.wrapping_add(*b).wrapping_mul(17))
}
