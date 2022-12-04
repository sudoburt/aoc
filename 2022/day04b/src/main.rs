fn main() {
    let input: Vec<&str> = include_str!("../input")
        .lines()
        .collect();

    let mut total = 0;

    for line in input {
        let line: Vec<&str> = line.split(',').collect();

        let first: Vec<u32> = line[0].split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let second: Vec<u32> = line[1].split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if !(first[0] > second[1] || first[1] < second[0]) {
            total += 1;
        }
    }

    println!("The total number of overlapping sections is {}", total);

}
