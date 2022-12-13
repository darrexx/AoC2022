fn main() {
    let input = include_str!("../input.txt");

    let instructions = input.lines().map(|x| Instruction::from(x)).collect();

    let part1 = run_instructions(&instructions);
    println!("Part1: {}", part1);

    let mut cpu = CPU {
        crt: Vec::new(),
        cycle: 1,
        x: 1,
    };
    (0..6).for_each(|_| {
        cpu.crt.push((0..40).map(|_| false).collect());
    });
    cpu.run(&instructions);

    println!("Part2:");
    (0..6).for_each(|i| {
        for pixel in &cpu.crt[i] {
            if *pixel {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    });
}

#[derive(Debug)]
enum Instruction {
    Add(isize),
    Noop,
}

impl From<&str> for Instruction {
    fn from(str: &str) -> Self {
        let instruction = str.split_whitespace().collect::<Vec<_>>();
        match instruction[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Add(instruction[1].parse().unwrap()),
            _ => panic!(),
        }
    }
}

fn run_instructions(instructions: &Vec<Instruction>) -> isize {
    let mut relevant_cycles = vec![20, 60, 100, 140, 180, 220].into_iter().peekable();
    let mut current_cycle = 0;
    let mut x = 1;
    let mut signal_strengths = Vec::new();
    for instruction in instructions {
        match instruction {
            Instruction::Add(to_add) => {
                if &(current_cycle + 2) >= relevant_cycles.peek().unwrap_or(&isize::MAX) {
                    let a = x * relevant_cycles.next().unwrap();
                    signal_strengths.push(a);
                }
                x += to_add;
                current_cycle += 2;
            }
            Instruction::Noop => {
                if &(current_cycle + 1) > relevant_cycles.peek().unwrap_or(&isize::MAX) {
                    let a = x * relevant_cycles.next().unwrap();
                    signal_strengths.push(a);
                }
                current_cycle += 1
            }
        }
    }
    signal_strengths.into_iter().sum()
}

struct CPU {
    x: isize,
    crt: Vec<Vec<bool>>,
    cycle: usize,
}

impl CPU {
    pub fn run(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            match instruction {
                Instruction::Add(to_add) => {
                    self.draw();
                    self.cycle += 1;
                    self.draw();
                    self.cycle += 1;
                    self.x += to_add;
                }
                Instruction::Noop => {
                    self.draw();
                    self.cycle += 1;
                }
            }
        }
    }

    fn draw(&mut self) {
        if (self.x - 1..=self.x + 1).contains(&isize::try_from((self.cycle - 1) % 40).unwrap()) {
            let crt_row = (self.cycle - 1) / 40;
            let pixel = (self.cycle - 1) % 40;
            self.crt[crt_row][pixel] = true;
        }
    }
}
