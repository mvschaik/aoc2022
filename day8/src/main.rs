use std::io::{self, BufRead};

fn main() {
    let mut field: Vec<Vec<i32>> = vec![];

    for line in io::stdin().lock().lines() {
        field.push(line.unwrap().chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
    }

    let nrows = field.len();
    let ncols = field[0].len();

    let mut visible = vec![vec![false; ncols]; nrows];
    let mut scenic = vec![vec![1; ncols]; nrows];

    for row in 0..nrows {
        let mut lhighest = -1;
        let mut rhighest = -1;
        let mut lheight2pos: Vec<usize> = vec![0; 10];
        let mut rheight2pos: Vec<usize> = vec![0; 10];
        for col in 0..ncols {
            // Step 1: pass from left to right.
            if field[row][col] > lhighest {
                visible[row][col] = true;
                lhighest = field[row][col];
            }

            // Step 1: Pass from right to left.
            let rcol = ncols - 1 - col;
            if field[row][rcol] > rhighest {
                visible[row][rcol] = true;
                rhighest = field[row][rcol];
            }

            // Step 2: Pass from left to right.
            scenic[row][col] *= col - lheight2pos[field[row][col] as usize];
            for i in 0..=field[row][col] {
                lheight2pos[i as usize] = col;
            }

            // Step 2: Pass from right to left.
            scenic[row][rcol] *= col - rheight2pos[field[row][rcol] as usize];
            for i in 0..=field[row][rcol] {
                rheight2pos[i as usize] = col;
            }
        }
    }
    for col in 0..ncols {
        let mut dhighest = -1;
        let mut uhighest = -1;
        let mut dheight2pos: Vec<usize> = vec![0; 10];
        let mut uheight2pos: Vec<usize> = vec![0; 10];
        for row in 0..nrows {
            // Step 1: down.
            if field[row][col] > dhighest {
                visible[row][col] = true;
                dhighest = field[row][col];
            }
            // Step 1: up.
            let urow = nrows - 1 - row;
            if field[urow][col] > uhighest {
                visible[urow][col] = true;
                uhighest = field[urow][col];
            }

            // Step 2: down.
            scenic[row][col] *= row - dheight2pos[field[row][col] as usize];
            for i in 0..=field[row][col] {
                dheight2pos[i as usize] = row;
            }

            // Step 2: up.
            scenic[urow][col] *= row - uheight2pos[field[urow][col] as usize];
            for i in 0..=field[urow][col] {
                uheight2pos[i as usize] = row;
            }
        }
    }

    let nvisible: usize = visible.iter().map(|row| row.iter().filter(|v| **v).count()).sum();
    let maxscore = *scenic.iter().map(|row| row.into_iter().max().unwrap()).max().unwrap();
    println!("Step1: {} visible", nvisible);
    println!("Step2: score: {}", maxscore);
}
