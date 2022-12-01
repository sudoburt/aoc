fn main() {
    let input = include_str!("../input")
        .lines()
        .map(|n| n.parse().unwrap_or(0))
        .collect::<Vec<u32>>();

    let mut calories: Vec<u32> = vec![];
    let mut curr_calories: u32 = 0;

    for item in input {
        match item {
            0 => {
                calories.push(curr_calories);
                curr_calories = 0;
            },
            item => {
                curr_calories += item;
            }
        }
    }

    calories.sort();
    calories.reverse();

    println!("The top 3 elves are carrying {} calories combined", 
        calories[0] + calories[1] + calories[2]
    );
}
