use itertools::Itertools;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_input() -> Option<(Vec<Vec<char>>, Vec<Instruction>)> {
    let input = std::fs::read_to_string("src/input.txt").ok()?;
    let (left, instructions_str) = input.split_once("\n\n")?;
    let (stacks_str, platforms) = left.rsplit_once('\n')?;

    // parse crates
    let num_stacks = platforms.split_whitespace().last()?.parse().ok()?;
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in stacks_str.lines().rev() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let second = chunk.nth(1)?;
            if second.is_alphabetic() {
                stacks[idx].push(second);
            }
        }
    }

    // parse instructions
    let mut instructions = Vec::new();
    for line in instructions_str.lines() {
        let rest = line.strip_prefix("move ")?;
        let (amount, rest) = rest.split_once(" from ")?;
        let (from, to) = rest.split_once(" to ")?;
        let instruction = Instruction {
            amount: amount.parse().ok()?,
            from: from.parse::<usize>().ok()? - 1,
            to: to.parse::<usize>().ok()? - 1,
        };
        instructions.push(instruction);
    }

    Some((stacks, instructions))
}

pub fn part_1() -> String {
    let (mut stacks, instructions) = parse_input().unwrap();
    for Instruction { amount, from, to } in instructions {
        for _ in 0..amount {
            if let Some(removed) = stacks[from].pop() {
                stacks[to].push(removed);
            }
        }
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

pub fn part_2() -> String {
    let (mut stacks, instructions) = parse_input().unwrap();
    for Instruction { amount, from, to } in instructions {
        let from_stack_len = stacks[from].len();
        let removed = stacks[from].split_off(from_stack_len - amount);
        stacks[to].extend(removed);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

fn main() {
    println!("{:?}", part_1());
    println!("{:?}", part_2());
}
