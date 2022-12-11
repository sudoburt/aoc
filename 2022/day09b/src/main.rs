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
    
    pub fn translate(&mut self, following: Coord) {
        if following.y - self.pos.y > 1 {
            self.prev.insert(Coord::from(self.pos.x, self.pos.y));
            self.pos.y += 1;
            if following.x > self.pos.x {
                self.pos.x += 1;
            }
            else if following.x < self.pos.x {
                self.pos.x -= 1;
            }
        }
        else if following.y - self.pos.y < -1 {
            self.prev.insert(Coord::from(self.pos.x, self.pos.y));
            self.pos.y -= 1;
            if following.x > self.pos.x {
                self.pos.x += 1;
            }
            else if following.x < self.pos.x {
                self.pos.x -= 1;
            }
        }
        else if following.x - self.pos.x > 1 {
            self.prev.insert(Coord::from(self.pos.x, self.pos.y));
            self.pos.x += 1;
            if following.y > self.pos.y {
                self.pos.y += 1;
            }
            else if following.y < self.pos.y {
                self.pos.y -= 1;
            }
        }
        else if  following.x - self.pos.x < -1 {
            self.prev.insert(Coord::from(self.pos.x, self.pos.y));
            self.pos.x -= 1;
            if following.y > self.pos.y {
                self.pos.y += 1;
            }
            else if following.y < self.pos.y {
                self.pos.y -= 1;
            }
        }
    }
}

#[derive(Debug)]
struct Rope {
    head: Head,
    tails: [Tail; 9]
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            head: Head::new(),
            tails: [
                Tail::new(),
                Tail::new(),
                Tail::new(),
                Tail::new(),
                Tail::new(),
                Tail::new(),
                Tail::new(),
                Tail::new(),
                Tail::new(),
            ]
        }
    }

    pub fn translate(&mut self, translation: &Direction) {
        match translation {
            Direction::Up(amount) => {
                for _ in 0..*amount {
                    self.head.pos.y += 1;
                    let mut following = Coord {
                        x: self.head.pos.x,
                        y: self.head.pos.y
                    };
                    for i in 0..self.tails.len() {
                        self.tails[i].translate(following);
                        following = Coord {
                            x: self.tails[i].pos.x,
                            y: self.tails[i].pos.y
                        };
                    }
                }
            }
            Direction::Down(amount) => {
                for _ in 0..*amount {
                    self.head.pos.y -= 1;
                    let mut following = Coord {
                        x: self.head.pos.x,
                        y: self.head.pos.y
                    };
                    for i in 0..self.tails.len() {
                        self.tails[i].translate(following);
                        following = Coord {
                            x: self.tails[i].pos.x,
                            y: self.tails[i].pos.y
                        };
                    }
                }
            }
            Direction::Left(amount) => {
                for _ in 0..*amount {
                    self.head.pos.x -= 1;
                    let mut following = Coord {
                        x: self.head.pos.x,
                        y: self.head.pos.y
                    };
                    for i in 0..self.tails.len() {
                        self.tails[i].translate(following);
                        following = Coord {
                            x: self.tails[i].pos.x,
                            y: self.tails[i].pos.y
                        };
                    }
                }
            }
            Direction::Right(amount) => {
                for _ in 0..*amount {
                    self.head.pos.x += 1;
                    let mut following = Coord {
                        x: self.head.pos.x,
                        y: self.head.pos.y
                    };
                    for i in 0..self.tails.len() {
                        self.tails[i].translate(following);
                        following = Coord {
                            x: self.tails[i].pos.x,
                            y: self.tails[i].pos.y
                        };
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        let mut coords = self.tails.iter().map(|t| (t.pos.x, t.pos.y)).collect::<Vec<(i32, i32)>>();

        coords.insert(0, (self.head.pos.x, self.head.pos.y));
         
        for y in (-20..20).rev() {
            for x in -20..20 {
                if coords.contains(&(x,y)) {
                    print!("{}", coords.iter().position(|c| c == &(x,y)).unwrap());
                }
                else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
    }

    pub fn draw_prev(&self) {
        let coords = self.tails[8].prev.iter().map(|c| (c.x, c.y)).collect::<Vec<(i32, i32)>>();

        for y in (-200..50).rev() {
            for x in -400..100 {
                if coords.contains(&(x,y)) {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
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

    rope.tails[8]
        .prev
        .insert(
            Coord {
                x: rope.tails[8].pos.x, 
                y: rope.tails[8].pos.y
            }
        );

    println!("The tail has visited {} different locations.",
        rope.tails[8].prev.len());
}
