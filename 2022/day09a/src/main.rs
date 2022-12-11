use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    pub fn new() -> Coord {
        Coord {
            x: 0,
            y: 0
        }
    }
    pub fn from(x: i32, y: i32) -> Coord {
        Coord {
            x,
            y
        }
    }
}

#[derive(Debug)]
struct Head {
    pos: Coord,
}

impl Head {
    pub fn new() -> Head {
        Head {
            pos: Coord::new()
        }
    }

}

#[derive(Debug)]
struct Tail {
    pos: Coord,
    prev: HashSet<Coord>
}

impl Tail {
    pub fn new() -> Tail {
        Tail {
            pos: Coord::new(),
            prev: HashSet::new()
        }
    }
}

#[derive(Debug)]
struct Rope {
    head: Head,
    tail: Tail
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            head: Head::new(),
            tail: Tail::new()
        }
    }

    pub fn translate(&mut self, translation: &Direction) {
        match translation {
            Direction::Up(amount) => {
                for _ in 0..*amount {
                    self.head.pos.y += 1;
                    if (self.head.pos.y - self.tail.pos.y).abs() > 1 {
                        self.tail.prev.insert(Coord::from(self.tail.pos.x, self.tail.pos.y));
                        self.tail.pos.y += 1;
                        self.tail.pos.x = self.head.pos.x;
                    }
                }
            }
            Direction::Down(amount) => {
                for _ in 0..*amount {
                    self.head.pos.y -= 1;
                    if (self.head.pos.y - self.tail.pos.y).abs() > 1 {
                        self.tail.prev.insert(Coord::from(self.tail.pos.x, self.tail.pos.y));
                        self.tail.pos.y -= 1;
                        self.tail.pos.x = self.head.pos.x;
                    }
                }
            }
            Direction::Left(amount) => {
                for _ in 0..*amount {
                    self.head.pos.x -= 1;
                    if (self.head.pos.x - self.tail.pos.x).abs() > 1 {
                        self.tail.prev.insert(Coord::from(self.tail.pos.x, self.tail.pos.y));
                        self.tail.pos.x -= 1;
                        self.tail.pos.y = self.head.pos.y;
                    }
                }
            }
            Direction::Right(amount) => {
                for _ in 0..*amount {
                    self.head.pos.x += 1;
                    if (self.head.pos.x - self.tail.pos.x).abs() > 1 {
                        self.tail.prev.insert(Coord::from(self.tail.pos.x, self.tail.pos.y));
                        self.tail.pos.x += 1;
                        self.tail.pos.y = self.head.pos.y;
                    }
                }
            }
        }
    }
}

fn main() {
    let directions: Vec<Direction> = include_str!("../input")
        .lines()
        .map(|l| {
            let line = l.split_whitespace().collect::<Vec<&str>>();
            match line[0] {
                "U" => Direction::Up(line[1].parse().unwrap()),
                "D" => Direction::Down(line[1].parse().unwrap()),
                "L" => Direction::Left(line[1].parse().unwrap()),
                "R" => Direction::Right(line[1].parse().unwrap()),
                _ => unreachable!()
            }
        })
        .collect();

    let mut rope = Rope::new();

    for direction in directions {
        rope.translate(&direction);
    }

    rope.tail.prev.insert(Coord { x: rope.tail.pos.x, y: rope.tail.pos.y });

    println!("The tail has visited {} different locations.", rope.tail.prev.len());
}
