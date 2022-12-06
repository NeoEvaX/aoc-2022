fn main() {
    let input = include_str!("./input.txt").lines();

    let mut score: String = "".to_owned();
    let mut score_part2: String = "".to_owned();

    let mut box_count: u32 = 0;
    let mut unsorted_cargo: Vec<Vec<&str>> = Default::default();
    let mut cargo: Vec<Vec<&str>> = Default::default();

    // Deconstruct into vector of vector of chars
    // These start at bottom of map and move up
    // Each line at a time.
    for line in input.clone().rev() {
        if line.contains("move") || line.is_empty() {
            continue;
        }
        let mut cargo_boxes: Vec<&str> = line.split_whitespace().collect();

        // Gets total count of columns and builds the eventual cargo size
        if cargo_boxes[0] == "1" {
            box_count = cargo_boxes.len() as u32;
            cargo_boxes.clear();
            let vec = Vec::with_capacity(box_count as usize);
            cargo = vec![vec; box_count as usize];
            continue;
        }

        // if row is broken up with a space
        if cargo_boxes.len() != box_count as usize {
            cargo_boxes = line.split("    ").collect();

            if cargo_boxes.len() != box_count as usize {
                let mut fixed_row: Vec<&str> = Default::default();
                for column in cargo_boxes {
                    if column.contains(' ') {
                        fixed_row.append(&mut column.split(' ').collect());
                    } else {
                        fixed_row.push(column);
                    }
                }
                cargo_boxes = fixed_row;
            }
        }
        unsorted_cargo.push(cargo_boxes);
    }

    // Pivots the table to be the right direction for use.
    for column in unsorted_cargo {
        for (j, row) in column.iter().enumerate() {
            if row.is_empty() {
                continue;
            }
            cargo[j].push(&row[1..2]);
        }
    }

    // Do all the Moves with CrateMover 9000
    let mut cargo_check1 = cargo.clone();
    for line in input.clone() {
        if !line.contains("move") {
            continue;
        }

        let cargo_boxes: Vec<&str> = line.split_whitespace().collect();
        let move_amount: usize = cargo_boxes[1].parse().unwrap();
        let move_from: usize = cargo_boxes[3].parse().unwrap();
        let move_to: usize = cargo_boxes[5].parse().unwrap();
        let cargo_box_size = cargo_check1[move_from - 1].len();

        for n in 0..move_amount {
            let mut moved_boxes = cargo_check1[move_from - 1].split_off(cargo_box_size - n - 1);
            cargo_check1[move_to - 1].append(&mut moved_boxes);
        }
    }
    for column in cargo_check1 {
        score += column[column.len() - 1]
    }

    // Do all the Moves with CrateMover 9001
    let mut cargo_check2 = cargo.clone();
    for line in input.clone() {
        if !line.contains("move") {
            continue;
        }

        let cargo_boxes: Vec<&str> = line.split_whitespace().collect();
        let move_amount: usize = cargo_boxes[1].parse().unwrap();
        let move_from: usize = cargo_boxes[3].parse().unwrap();
        let move_to: usize = cargo_boxes[5].parse().unwrap();
        let cargo_box_size = cargo_check2[move_from - 1].len();

        for _n in 0..move_amount {
            let mut moved_boxes =
                cargo_check2[move_from - 1].split_off(cargo_box_size - move_amount);
            cargo_check2[move_to - 1].append(&mut moved_boxes);
        }
    }
    for column in cargo_check2 {
        score_part2 += column[column.len() - 1]
    }

    println!("Final Answer: {:?}", score);
    println!("Final Answer 2: {:?}", score_part2);
}
