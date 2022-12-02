enum Weapon {
    Rock,
    Paper,
    Scissors
}

impl Weapon {
    fn from(input: &str) -> Weapon {
        match input {
            "A" | "X" => Weapon::Rock,
            "B" | "Y" => Weapon::Paper,
            "C" | "Z" => Weapon::Scissors,
            _ => panic!("Input was not a valid weapon!"),
        }
    }

    fn fight(&self, other: &Weapon) -> u32 {
        match self {
            Weapon::Rock => 1 +
                match other {
                    Weapon::Rock => 3,
                    Weapon::Paper => 0,
                    Weapon::Scissors => 6,
                },
            Weapon::Paper => 2 +
                match other {
                    Weapon::Rock => 6,
                    Weapon::Paper => 3,
                    Weapon::Scissors => 0,
                },
            Weapon::Scissors => 3 +
                match other {
                    Weapon::Rock => 0,
                    Weapon::Paper => 6,
                    Weapon::Scissors => 3,
                },
        }
    }
}

fn main() {
    let input: Vec<Vec<&str>> = include_str!("../input")
        .lines()
        .map(|line| line.split(" ").collect())
        .collect();

    let mut score = 0;

    for turn in input {
        let our_weapon = Weapon::from(turn[1]);
        let their_weapon = Weapon::from(turn[0]);
        score += our_weapon.fight(&their_weapon);
    }

    println!("The score of the strategy guide would be {}", score);
}
