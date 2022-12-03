fn main() {
    let input = include_str!("./input.txt").lines();

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let mut score: u32 = 0;
    let mut score_part2: u32 = 0;

    for line in input.clone() {
        let rutsack: (&str, &str) = line.split_at(line.len() / 2);

        for item in rutsack.0.chars() {
            if rutsack.1.contains(item) {
                score += alphabet.iter().position(|&r| r == item).unwrap() as u32 + 1;
                break;
            }
        }
    }

    let mut group_number = 0;
    let mut group: Vec<&str> = Default::default();
    for line in input {
        group.push(line);
        group_number += 1;

        if group_number == 3 {
            for item in group[0].chars() {
                if group[1].contains(item) && group[2].contains(item) {
                    score_part2 += alphabet.iter().position(|&r| r == item).unwrap() as u32 + 1;
                    break;
                }
            }

            group_number = 0;
            group.clear();
        }
    }

    println!("Score: {}", score);
    println!("Score2: {}", score_part2)
}
