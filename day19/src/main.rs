use std::{
    cmp::{max, min},
    collections::HashMap,
};

type Part = [u64; 4];

struct Rule {
    condition: Option<Condition>,
    target: String,
}

impl Rule {
    fn applies(&self, part: &Part) -> bool {
        self.condition
            .as_ref()
            .map(|c| c.applies(part))
            .unwrap_or(true)
    }
}

struct Condition {
    index: usize,
    value: u64,
    comp: Comp,
}

enum Comp {
    LT,
    GT,
}

impl Condition {
    fn applies(&self, part: &Part) -> bool {
        match self.comp {
            Comp::LT => part[self.index] < self.value,
            Comp::GT => part[self.index] > self.value,
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let parts: Vec<Part> = parts
        .lines()
        .map(|l| {
            l.trim_matches(['{', '}'])
                .split(',')
                .map(|s| s[2..].parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let workflows: HashMap<_, _> = workflows
        .lines()
        .map(|l| {
            let (name, rules) = l.trim_end_matches('}').split_once('{').unwrap();
            let rules: Vec<Rule> = rules
                .split(',')
                .map(|r| {
                    if let Some((cond, target)) = r.split_once(':') {
                        let cat = cond.as_bytes()[0];
                        let index = b"xmas".iter().position(|c| *c == cat).unwrap();
                        let op = if &cond[1..2] == "<" {
                            Comp::LT
                        } else {
                            Comp::GT
                        };

                        Rule {
                            condition: Some(Condition {
                                index,
                                comp: op,
                                value: cond[2..].parse().unwrap(),
                            }),
                            target: target.to_string(),
                        }
                    } else {
                        Rule {
                            condition: None,
                            target: r.to_string(),
                        }
                    }
                })
                .collect();

            (name, rules)
        })
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&workflows, &parts),
        part_2(&workflows, "in", [(1, 4001); 4]),
    );
}

fn part_1(workflows: &HashMap<&str, Vec<Rule>>, parts: &[Part]) -> u64 {
    parts
        .iter()
        .filter(|p| {
            let mut wf = &workflows["in"];

            loop {
                for rule in wf {
                    if rule.applies(*p) {
                        match rule.target.as_str() {
                            "R" => return false,
                            "A" => return true,
                            target => {
                                wf = &workflows[target];
                                break;
                            }
                        }
                    }
                }
            }
        })
        .flatten()
        .sum()
}

fn part_2(workflows: &HashMap<&str, Vec<Rule>>, rule: &str, mut ranges: [(u64, u64); 4]) -> u64 {
    if rule == "R" {
        return 0;
    }

    let product = ranges.iter().map(|r| r.1.saturating_sub(r.0)).product();

    if rule == "A" || product == 0 {
        return product;
    }

    workflows[rule]
        .iter()
        .map(|r| {
            if let Some(cond) = &r.condition {
                let mut matched = ranges.clone();
                let rm = &mut matched[cond.index];
                let rn = &mut ranges[cond.index];

                match cond.comp {
                    Comp::LT => {
                        rm.1 = min(rm.1, cond.value);
                        rn.0 = max(rn.0, cond.value);
                    }
                    Comp::GT => {
                        rm.0 = max(rm.0, cond.value + 1);
                        rn.1 = min(rn.1, cond.value + 1);
                    }
                }

                part_2(&workflows, &r.target, matched)
            } else {
                part_2(workflows, &r.target, ranges.clone())
            }
        })
        .sum()
}
