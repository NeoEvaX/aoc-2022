fn main() {
    let input = include_str!("./input.txt").lines();

    let mut score: u32 = 0;
    let mut part2_score: u32 = 0;

    for line in input {
        let game: Vec<&str> = line.split(' ').collect();
        let mut elf_hand: u32 = 0;
        let mut player_hand: u32 = 0;
        match game[0] {
            "A" => {
                elf_hand += 1;
            }
            "B" => {
                elf_hand += 2;
            }
            "C" => {
                elf_hand += 3;
            }
            _ => println!("Error"),
        }
        match game[1] {
            "X" => {
                // Lose
                player_hand += 1;
            }
            "Y" => {
                //Draw
                player_hand += 2;
            }
            "Z" => {
                //Win
                player_hand += 3;
            }
            _ => println!("Error"),
        }

        score += player_hand;

        match player_hand {
            1 => {
                if player_hand == elf_hand {
                    score += 3;
                } else if elf_hand == 3 {
                    score += 6;
                }
                //Lose
                match elf_hand {
                    1 => part2_score += 3,
                    2 => part2_score += 1,
                    3 => part2_score += 2,
                    _ => {}
                }
            }
            2 => {
                if player_hand == elf_hand {
                    score += 3;
                } else if elf_hand == 1 {
                    score += 6;
                }
                //Draw
                match elf_hand {
                    1 => part2_score += 4,
                    2 => part2_score += 5,
                    3 => part2_score += 6,
                    _ => {}
                }
            }
            3 => {
                if player_hand == elf_hand {
                    score += 3;
                } else if elf_hand == 2 {
                    score += 6;
                }
                //Win
                match elf_hand {
                    1 => part2_score += 8,
                    2 => part2_score += 9,
                    3 => part2_score += 7,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    println!("Part 1: {score}");
    println!("Part 2: {part2_score}");
}
