fn main() {
    let input = include_str!("../input")
        .lines()
        .map(|n| n.parse().unwrap_or(0))
        .collect::<Vec<u128>>();

    let mut most_calories: u128 = 0;
    let mut curr_calories: u128 = 0;

    for item in input {
        match item {
            0 => {
                if curr_calories > most_calories {
                    most_calories = curr_calories;
                }
                curr_calories = 0;
            },
            item => {
                curr_calories += item;
            }
        }
    }

    println!("The elf carrying the most calories is carrying {} calories", most_calories);
}
