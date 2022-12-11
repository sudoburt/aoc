fn get_visibility(cc: i32, x: i32) -> String {
    if ((x+1)-(cc % 40)).abs() <= 1 {
        "#".to_string()
    }
    else {
        " ".to_string()
    }
}

fn print_crt(crt: &mut Vec<String>, cc: i32, x: i32) {
    if cc % 40 == 0 {
        crt[(cc/41) as usize].push_str(get_visibility(cc, x).as_str());
    }
    else {
        crt[(cc/40) as usize].push_str(get_visibility(cc, x).as_str());
    }
}

fn print_sprite_position(x: i32) {
    for i in 0..40 {
        if (x-i).abs() <= 1 {
        }
        else {
        }
    }
}

fn main() {
    let lines: Vec<Vec<&str>> = include_str!("../input")
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let mut cc = 1;

    let mut x = 1;
    let mut buffer: (i32, i32);

    let mut crt: Vec<String> = vec![
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ];

    for line in lines {
        print_sprite_position(x);
        match line[0] {
            "addx" => buffer = (1, line[1].parse::<i32>().unwrap()),
            _ => buffer = (0,0),
        }

        while buffer.0 > 0 {
            buffer.0 -= 1;
            print_crt(&mut crt, cc, x);
            cc += 1;
        }

        print_crt(&mut crt, cc, x);
        x += buffer.1;
        cc += 1;
    }

    for line in crt {
        println!("{}", line);
    }
}
