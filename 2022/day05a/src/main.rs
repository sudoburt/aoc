#[derive(Debug)]
struct Stack {
    boxes: Vec<Vec<char>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            boxes: vec![]
        }
    }

    fn add(&mut self, i: usize, char: char) {
        while i >= self.boxes.len() {
            self.boxes.push(vec![]);
        }
        self.boxes[i].insert(0, char);
    }

    pub fn add_line(&mut self, input: &str) {
        let line: Vec<char> = input
            .replace("    ", " 000")
            .replace(" ","")
            .chars()
            .collect::<Vec<char>>();

        for (i, r#box) in line.chunks(3).enumerate() {
            if r#box != &['0', '0', '0'] {
                self.add(i, r#box[1]);
            }
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        for _i in 0..instruction.amount {
            let swap = self.boxes[instruction.from].pop().unwrap();
            self.boxes[instruction.to].push(swap);
        }
    }

    pub fn print_tops(&self) {
        print!("The tops of the stacks are ");
        for r#box in &self.boxes {
            print!("{}", r#box[r#box.len()-1]);
        }
        println!();
    }
}

#[derive(Debug)]
struct Instruction {
    amount: u8,
    from: usize,
    to: usize
}

impl Instruction {
    pub fn from(input: &str) -> Instruction {
        let params: Vec<&str> = input.split(' ').collect();

        Instruction {
            amount: params[1].parse::<u8>().unwrap(),
            from: params[3].parse::<usize>().unwrap()-1,
            to: params[5].parse::<usize>().unwrap()-1
        }
    }
}


fn main() {
    let input: Vec<&str> = include_str!("../input")
        .lines()
        .collect();

    let mut stack = Stack::new();
    let mut instructions: Vec<Instruction> = vec![];

    for line in &input {
        if line.contains('[') {
            stack.add_line(line);
        }
        else if line.contains("move") {
            instructions.push(Instruction::from(line));
        }
    }

    for instruction in instructions {
        stack.execute_instruction(instruction);
    }

    stack.print_tops();

}
