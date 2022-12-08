use anyhow::Result;
use std::collections::HashSet;

pub fn run1(input: &str) -> Result<()> {
    let mut visibles = HashSet::new();
    let field_size = input.lines().count();
    let mut col_max_heights = vec![];

    for (row, line) in input.lines().enumerate() {
        // Check if it can be seen from the left
        let mut max_height = 0;
        let mut col = 0;
        for height in line.chars() {
            let height: i32 = height.to_string().parse()?;
            if row == 0 || col == 0 {
                visibles.insert((col, row));
            }

            if height as i32 > max_height {
                visibles.insert((col, row));
                max_height = height;
            }

            // Update each col max_height and check if visible from the top
            if row == 0 {
                col_max_heights.push(height);
            } else if height > col_max_heights[col] {
                visibles.insert((col, row));
                col_max_heights[col] = height;
            }

            col += 1;
        }

        // Check if it can be seen from the right
        max_height = 0;
        for height in line.chars().rev() {
            col -= 1;
            let height: i32 = height.to_string().parse()?;
            if row == field_size - 1 || col == field_size - 1 {
                visibles.insert((col, row));
            }

            if height as i32 > max_height {
                visibles.insert((col, row));
                max_height = height;
            }
        }
    }

    // Finally, check if visible from the bottom
    col_max_heights.clear();
    for (row, line) in input.lines().rev().enumerate() {
        for (col, height) in line.chars().enumerate() {
            let height: i32 = height.to_string().parse()?;
            // Update each col max_height and check if visible from the top
            if row == 0 {
                col_max_heights.push(height);
                //println!("{:?}", col_max_heights);
            } else if height > col_max_heights[col] {
                //println!("{} > {} ?", height, col_max_heights[col]);
                visibles.insert((col, field_size - row - 1));
                col_max_heights[col] = height;
            }
        }
    }

    println!("Visible trees: {}", visibles.len());

    Ok(())
}

fn scenic_score(forest: &[Vec<usize>], row: usize, col: usize) -> usize {
    let mut current = 0;
    let mut score = 1;
    let tree = forest[row][col];

    // Check up
    for tree_row in (0..row).rev() {
        current += 1;
        if forest[tree_row][col] >= tree {
            break;
        }
    }

    score *= current;

    // Check down
    current = 0;
    for tree_row in (row + 1..forest.len()).rev() {
        current += 1;
        if forest[tree_row][col] >= tree {
            break;
        }
    }

    score *= current;

    // Check left
    current = 0;
    for tree_col in (0..col).rev() {
        current += 1;
        if forest[row][tree_col] >= tree {
            break;
        }
    }

    score *= current;

    // Check right
    current = 0;
    for tree_col in col + 1..forest.len() {
        current += 1;
        if forest[row][tree_col] >= tree {
            break;
        }
    }

    score * current
}

fn run2(input: &str) -> Result<()> {
    let mut max_score = 0;
    let field_size = input.lines().count();
    let forest: Vec<Vec<usize>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    for row in 0..field_size {
        for col in 0..field_size {
            let score = scenic_score(&forest, row, col);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Best tree: {}", max_score);

    Ok(())
}

pub fn run(input: &str) -> Result<()> {
    run1(input)?;
    run2(input)?;

    Ok(())
}
