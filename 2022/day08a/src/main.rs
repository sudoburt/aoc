fn main() {
    let lines: Vec<&str> = include_str!("../input")
        .lines()
        .collect();

    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total = 2 * grid.len() + 2 * grid[0].len() - 4;

    for row in 1..grid.len()-1 {
        for col in 1..grid[0].len()-1 {
            let cur = grid[row][col];

            if cur > *grid[row][..col].iter().max().unwrap() {
                total += 1;
            }

            else if cur > *grid[row][col+1..].iter().max().unwrap() {
                total += 1;
            }

            else if cur > grid[..row].iter().map(|r| r[col]).max().unwrap() {
                total += 1;
            }

            else if cur > grid[row+1..].iter().map(|r| r[col]).max().unwrap() {
                total += 1;
            }
        }
    }

    println!("The total amount of visible trees is {}", total);

}
