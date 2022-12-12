#[derive(Debug)]
enum Operation {
    Add(u32),
    Subtract(u32),
    Multiply(u32),
    Divide(u32),
    Square,
    Double
}


#[derive(Debug)]
enum Test {
    Divisible(u32)
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: Test,
    true_throw: usize,
    false_throw: usize,
    inspection_count: usize
}

impl Monkey {
    pub fn from(lines: Vec<&str>) -> Monkey {
        let items = lines[1][18..]
                .split(", ")
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

        let operation_line = lines[2][23..]
            .split_whitespace()
            .collect::<Vec<&str>>();

        let operation = match operation_line[0] {
            "+" => {
                match operation_line[1] {
                    "old" => Operation::Double,
                    num => Operation::Add(num.parse::<u32>().unwrap()),
                }
            },
            "-" => Operation::Subtract(operation_line[1].parse::<u32>().unwrap()),
            "*" => {
                match operation_line[1] {
                    "old" => Operation::Square,
                    num => Operation::Multiply(num.parse::<u32>().unwrap()),
                }
            }
            "/" => Operation::Divide(operation_line[1].parse::<u32>().unwrap()),
            _ => unreachable!()
        };

        let test_line = lines[3][8..]
            .split(" by ")
            .collect::<Vec<&str>>();

        let test = match test_line[0] {
            "divisible" => Test::Divisible(test_line[1].parse::<u32>().unwrap()),
            _ => unreachable!()
        };

        let true_throw = lines[4][29..]
            .parse::<usize>()
            .unwrap();

        let false_throw = lines[5][30..]
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            operation,
            test,
            true_throw,
            false_throw,
            inspection_count: 0
        }
    }

    pub fn take_turn(&mut self) -> Vec<(usize, u32)> {
        let mut thrown_items = vec![];
        for item in &self.items {
            self.inspection_count += 1;
            let new_worry_level = self.perform_operation(*item) / 3 as u32;
            let test = self.perform_test(new_worry_level);
            if test {
                thrown_items.push((self.true_throw, new_worry_level));
            }
            else {
                thrown_items.push((self.false_throw, new_worry_level));
            }
        }
        self.items.clear();
        thrown_items
    }

    pub fn catch(&mut self, item: u32) {
        self.items.push(item);
    }
    
    fn perform_operation(&self, val: u32) -> u32 {
        match self.operation {
            Operation::Add(num) => val + num,
            Operation::Subtract(num) => val - num,
            Operation::Multiply(num) => val * num,
            Operation::Divide(num) => val / num,
            Operation::Square => val * val,
            Operation::Double => val + val,
        }
    }

    fn perform_test(&self, val: u32) -> bool {
        match self.test {
            Test::Divisible(num) => val % num == 0,
        }
    }
}

fn main() {
    let input: Vec<Vec<&str>> = include_str!("../input")
        .split("\n\n")
        .map(|l| l.split("\n").collect())
        .collect::<Vec<Vec<&str>>>();

    let mut monkeys: Vec<Monkey> = vec![];

    for lines in input {
        monkeys.push(Monkey::from(lines));
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].take_turn();
            for (destination, item) in thrown_items {
                monkeys[destination].catch(item);
            }
        }
    }

    let mut inspection_counts: Vec<usize> = monkeys
        .iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<usize>>();

    inspection_counts.sort();

    let monkey_business: usize = inspection_counts
        .iter()
        .rev()
        .take(2)
        .product();

    println!("The total monkey business is {}", monkey_business);
}
