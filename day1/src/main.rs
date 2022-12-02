fn main() {
    let input = include_str!("./input.txt");

    let gnomes = input.split("\n\n");

    let mut held_calories: Vec<u32> = gnomes
        .map(|calories| {
            calories
                .split('\n')
                .flat_map(|num| num.parse::<u32>())
                .sum()
        })
        .collect();

    held_calories.sort_by(|a, b| b.cmp(a));

    println!("Answer 1: {:?}", held_calories[0]);

    println!(
        "Answer 2: {:?}",
        held_calories[0] + held_calories[1] + held_calories[2]
    );
    println!(
        "Answer 2-2: {:?}",
        held_calories.iter().take(3).sum::<u32>()
    );
}
