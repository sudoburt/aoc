fn main() {
    let input: Vec<&str> = include_str!("../input")
        .lines()
        .collect();

    let mut priorities: Vec<u32> = vec![];

    for line in input {
        let first_half = &line[..line.len()/2];
        let second_half = &line[line.len()/2..];
        let item: char = first_half
            .chars()
            .filter(|&c| second_half.contains(c))
            .nth(0)
            .unwrap();

        match item {
            'a'..='z' => priorities.push(item as u32 - 'a' as u32 + 1),
            'A'..='Z' => priorities.push(item as u32 - 'A' as u32 + 27),
            _ => unreachable!("Recieved non-alphabetic character!")
        }
    }

    println!("The sum of the priorities of the item types is {}", 
        priorities.iter().sum::<u32>()
    );
}
