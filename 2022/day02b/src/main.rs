enum Outcome {
    Lose,
    Draw,
    Win
}

impl Outcome {
    fn from(input: &str) -> Outcome {
        match input {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Input was not a valid outcome!"),
        }
    }
}

enum Weapon {
    Rock,
    Paper,
    Scissors
}

impl Weapon {
    fn from(input: &str) -> Weapon {
        match input {
            "A" => Weapon::Rock,
            "B" => Weapon::Paper,
            "C" => Weapon::Scissors,
            _ => panic!("Input was not a valid weapon!"),
        }
    }

}

fn fight(weapon: &Weapon, desired_outcome: &Outcome) -> u32 {
    match weapon {
        Weapon::Rock => 
            match desired_outcome {
                Outcome::Lose => 3,
                Outcome::Draw => 4,
                Outcome::Win => 8,
            },
        Weapon::Paper => 
            match desired_outcome {
                Outcome::Lose => 1,
                Outcome::Draw => 5,
                Outcome::Win => 9,
            },
        Weapon::Scissors => 
            match desired_outcome {
                Outcome::Lose => 2,
                Outcome::Draw => 6,
                Outcome::Win => 7,
            },
    }
}

fn main() {
    let input: Vec<Vec<&str>> = include_str!("../input")
        .lines()
        .map(|line| line.split(" ").collect())
        .collect();

    let mut score = 0;

    for turn in input {
        let their_weapon = Weapon::from(turn[0]);
        let desired_outcome = Outcome::from(turn[1]);
        let new_score = fight(&their_weapon, &desired_outcome);
        score += new_score;
    }

    println!("The score of the strategy guide would be {}", score);
}
