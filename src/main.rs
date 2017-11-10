use std::io::{self, Read};

enum Operand {
    Next,
    Prev,
    Increment,
    Decrement,
    Print,
    Read,
    LoopBegin,
    LoopEnd,
}

struct Machine {
    pointer: usize,
    memory: Vec<u8>,
    operands: Vec<Operand>,
    operand_index: usize,
}

impl Machine {
    const INITIAL_SIZE: usize = 300000;

    fn new(operands: Vec<Operand>) -> Self {
        Machine {
            pointer: 0,
            memory: vec![0; Machine::INITIAL_SIZE],
            operands: operands,
            operand_index: 0,
        }
    }

    fn get_value(&self) -> u8 {
        self.memory[self.pointer]
    }

    fn get_operand(&self) -> &Operand {
        &self.operands[self.operand_index]
    }

    fn is_finished(&self) -> bool {
        self.operand_index < self.operands.len()
    }

    fn run(&mut self) {
        while self.is_finished() {
            match self.get_operand() {
                &Operand::Next => self.pointer += 1,
                &Operand::Prev => self.pointer -= 1,
                &Operand::Increment => self.memory[self.pointer] += 1,
                &Operand::Decrement => self.memory[self.pointer] -= 1,
                &Operand::Print => print!("{}", char::from(self.get_value())),
                &Operand::Read => {
                    let stdin = io::stdin();
                    let mut handle = stdin.lock();
                    handle.read(&mut self.memory[self.pointer..self.pointer+1]).unwrap();
                },
                &Operand::LoopBegin => {
                    if self.get_value() == 0 {
                        loop {
                            match self.get_operand() {
                                &Operand::LoopEnd => break,
                                _ => self.operand_index += 1,
                            }
                        }
                    }
                },
                &Operand::LoopEnd => {
                    if self.get_value() != 0 {
                        let mut num_inner_loops = 1;
                        while num_inner_loops != 0 {
                            self.operand_index -= 1;
                            match self.get_operand() {
                                &Operand::LoopBegin => num_inner_loops -= 1,
                                &Operand::LoopEnd => num_inner_loops += 1,
                                _ => {}
                            }
                        }
                    }
                },
            }
            self.operand_index += 1;
        }
    }
}

fn char2operand(c: char) -> Option<Operand> {
    match c {
        '>' => Some(Operand::Next),
        '<' => Some(Operand::Prev),
        '+' => Some(Operand::Increment),
        '-' => Some(Operand::Decrement),
        '.' => Some(Operand::Print),
        ',' => Some(Operand::Read),
        '[' => Some(Operand::LoopBegin),
        ']' => Some(Operand::LoopEnd),
        _ => None
    }
}

fn parse(commands: &String) -> Vec<Operand> {
    commands.chars()
            .filter_map(|c| char2operand(c))
            .collect()
}

fn main() {
    let commands = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    let mut machine = Machine::new(parse(&commands));
    machine.run();
}
