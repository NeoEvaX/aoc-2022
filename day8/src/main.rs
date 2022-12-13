use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut total = 0;
    let mut total_part2 = 0;

    let mut grid: Vec<Vec<u32>> = Default::default();

    (0..input.len()).for_each(|x| {
        grid.push(input[x].chars().flat_map(|ch| ch.to_digit(10)).collect());
    });

    // let file = include_str!("./input.txt");
    // let trees = file
    //     .lines()
    //     .map(|line| {
    //         return line
    //             .chars()
    //             .filter_map(|x| x.to_digit(10))
    //             .map(|x| x as usize)
    //             .collect::<Vec<usize>>();
    //     })
    //     .collect::<Vec<Vec<usize>>>();

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if x == 0 {
                total += 1;
                continue;
            }
            if x == input.len() - 1 {
                total += 1;
                continue;
            }
            if y == 0 {
                total += 1;
                continue;
            }
            if y == input[x].len() - 1 {
                total += 1;
                continue;
            }

            let grid_pivot = transpose(grid.clone());

            // Look to the side
            if grid[x][..y].iter().max().unwrap() < &grid[x][y]
                || grid[x][y + 1..].iter().max().unwrap() < &grid[x][y]
                || grid_pivot[y][..x].iter().max().unwrap() < &grid[x][y]
                || grid_pivot[y][x + 1..].iter().max().unwrap() < &grid[x][y]
            {
                total += 1;
            }

            let mut right = 0;
            let mut left = 0;
            let mut top = 0;
            let mut down = 0;
            // Check for items smaller than previous. Small trees hiding
            for item in grid[x][y + 1..].iter() {
                if item < &grid[x][y] {
                    right += 1;
                    continue;
                } else if item >= &grid[x][y] {
                    right += 1;
                    break;
                }
            }
            for item in grid[x][..y].iter().rev() {
                if item < &grid[x][y] {
                    left += 1;
                    continue;
                } else if item >= &grid[x][y] {
                    left += 1;
                    break;
                }
            }
            for item in grid_pivot[y][..x].iter().rev() {
                if item < &grid[x][y] {
                    top += 1;
                    continue;
                } else if item >= &grid[x][y] {
                    top += 1;
                    break;
                }
            }
            for item in grid_pivot[y][x + 1..].iter() {
                if item < &grid[x][y] {
                    down += 1;
                    continue;
                } else if item >= &grid[x][y] {
                    down += 1;
                    break;
                }
            }

            if right == 0 {
                right = 1;
            }
            if left == 0 {
                left = 1;
            }
            if top == 0 {
                top = 1;
            }
            if down == 0 {
                down = 1;
            }

            println!(
                "{} {} {} {} = {}",
                right,
                left,
                top,
                down,
                (right * left * top * down)
            );

            if total_part2 < (right * left * top * down) {
                total_part2 = right * left * top * down;
            }

            // println!("{} {} = {}", x, y, grid[x][y]);
            // println!("grid: {:?}", grid);
            // println!("grid Pivot: {:?}", grid_pivot);
            // println!("Max Left: {:?}", grid[x][..y].iter().max().unwrap());
            // println!("Max Right: {:?}", grid[x][y + 1..].iter().max().unwrap());

            // println!("Max Down: {:?}", grid_pivot[y][x + 1..].iter().max());
            // println!("Max UP: {:?}", grid_pivot[y][..x].iter().max());
        }
    }

    println!("Part 1: {:?}", total);
    println!("Part 2: {:?}", total_part2);

    Ok(())
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
