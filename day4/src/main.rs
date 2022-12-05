fn main() {
    let input = include_str!("./input.txt").lines();

    let mut score: u32 = 0;
    let mut score_part2: u32 = 0;

    for line in input.clone() {
        let cleaning_pairs: Vec<&str> = line.split(',').collect();

        let cleaning1: Vec<u32> = cleaning_pairs[0]
            .split('-')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let cleaning2: Vec<u32> = cleaning_pairs[1]
            .split('-')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        if (cleaning1[0] >= cleaning2[0] && cleaning1[1] <= cleaning2[1])
            || (cleaning2[0] >= cleaning1[0] && cleaning2[1] <= cleaning1[1])
        {
            score += 1;
        }

        if (cleaning1[0] >= cleaning2[0] && cleaning1[0] <= cleaning2[1])
            || (cleaning1[1] >= cleaning2[0] && cleaning1[1] <= cleaning2[1])
            || (cleaning2[0] >= cleaning1[0] && cleaning2[0] <= cleaning1[1])
            || (cleaning2[1] >= cleaning1[0] && cleaning2[1] <= cleaning1[1])
        {
            score_part2 += 1;
        }
    }

    println!("Score: {}", score);
    println!("Score2: {}", score_part2)
}
