fn main() {
    let lines: Vec<Vec<&str>> = include_str!("../input")
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let mut ss = 0;
    let mut cc = 1;

    let mut x = 1;
    let mut buffer: (i32, i32);

    for line in lines {
        match line[0] {
            "addx" => buffer = (1, line[1].parse::<i32>().unwrap()),
            _ => buffer = (0,0),
        }

        while buffer.0 > 0 {
            if cc >= 20 && (cc-20) % 40 == 0 && cc <= 220 {
                ss += cc * x;
            }

            buffer.0 -= 1;
            cc += 1;
        }

        if cc >= 20 && (cc-20) % 40 == 0 && cc <= 220 {
            ss += cc * x;
        }

        x += buffer.1;

        cc += 1;
    }

    println!("The total signal strength is {}", ss);
}
