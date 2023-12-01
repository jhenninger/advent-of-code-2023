use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let mut needles: HashMap<String, u32> = (0..=9).map(|i| (i.to_string(), i)).collect();

    let part_1: u32 = input.lines().map(|l| calibration_value(l, &needles)).sum();

    needles.insert("one".to_string(), 1);
    needles.insert("two".to_string(), 2);
    needles.insert("three".to_string(), 3);
    needles.insert("four".to_string(), 4);
    needles.insert("five".to_string(), 5);
    needles.insert("six".to_string(), 6);
    needles.insert("seven".to_string(), 7);
    needles.insert("eight".to_string(), 8);
    needles.insert("nine".to_string(), 9);

    let part_2: u32 = input.lines().map(|l| calibration_value(l, &needles)).sum();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}

fn calibration_value(haystack: &str, needles: &HashMap<String, u32>) -> u32 {
    let first = needles
        .keys()
        .filter_map(|needle| haystack.find(needle).map(|pos| (pos, needle)))
        .min_by_key(|(pos, _)| *pos)
        .unwrap()
        .1;

    let last = needles
        .keys()
        .filter_map(|needle| haystack.rfind(needle).map(|pos| (pos, needle)))
        .max_by_key(|(pos, _)| *pos)
        .unwrap()
        .1;

    needles[first] * 10 + needles[last]
}
