use ndarray::prelude::*;

const LEN: usize = 99;

fn main() {
    let lines: Vec<&str> = include_str!("../input")
        .lines()
        .collect();

    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut arr = Array2::<u32>::default((LEN, LEN));
    for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = grid[i][j];
        }
    }

    let mut largest = 0;

    for row in 0..LEN {
        for col in 0..LEN {
            let mut score = 1;

            score *= arr.slice(s![..row, col])
                .iter()
                .rev()
                .position(|h| h >= &arr[(row, col)])
                .unwrap_or(row.wrapping_sub(1))
                .wrapping_add(1);

            score *= arr.slice(s![row+1.., col])
                .iter()
                .position(|h| h >= &arr[(row, col)])
                .unwrap_or((LEN-1-row).wrapping_sub(1))
                .wrapping_add(1);

            score *= arr.slice(s![row, ..col])
                .iter()
                .rev()
                .position(|h| h >= &arr[(row, col)])
                .unwrap_or(col.wrapping_sub(1))
                .wrapping_add(1);

            score *= arr.slice(s![row, col+1..])
                .iter()
                .position(|h| h >= &arr[(row, col)])
                .unwrap_or((LEN-1-col).wrapping_sub(1))
                .wrapping_add(1);

            if score > largest {
                largest = score
            }
        }
    }

    println!("The largest possible scenic score is {}", largest);
}
