fn main() {
    let input: Vec<&str> = include_str!("../input")
        .lines()
        .collect();

    let mut priorities: Vec<u32> = vec![];

    for lines in input.chunks(3) {
        let item: char = lines[0]
            .chars()
            .filter(|&c| lines[1].contains(c) && lines[2].contains(c))
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
