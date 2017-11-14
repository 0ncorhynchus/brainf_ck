#![feature(test)]
extern crate test;

use std::iter::Peekable;
use std::io::{self, Read};

enum Operand {
    Next(usize),
    Prev(usize),
    Increment(u8),
    Decrement(u8),
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
                &Operand::Next(n) => self.pointer += n,
                &Operand::Prev(n) => self.pointer -= n,
                &Operand::Increment(n) => self.memory[self.pointer] += n,
                &Operand::Decrement(n) => self.memory[self.pointer] -= n,
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

#[derive(PartialEq)]
enum Command {
    Next,
    Prev,
    Increment,
    Decrement,
    Print,
    Read,
    LoopBegin,
    LoopEnd,
}

fn parse(commands: &String) -> Vec<Command> {
    commands.chars()
            .filter_map(|c| match c {
                '>' => Some(Command::Next),
                '<' => Some(Command::Prev),
                '+' => Some(Command::Increment),
                '-' => Some(Command::Decrement),
                '.' => Some(Command::Print),
                ',' => Some(Command::Read),
                '[' => Some(Command::LoopBegin),
                ']' => Some(Command::LoopEnd),
                _ => None
            })
            .collect()
}

fn count_command<I>(iter: &mut Peekable<I>, target: Command) -> usize
where I: Iterator<Item=Command>
{
    let mut count = 0;
    loop {
        if let Some(next) = iter.peek() {
            if next != &target {
                break;
            }
        } else {
            break;
        }
        count += 1;
        iter.next();
    }
    count
}

fn pass(commands: Vec<Command>) -> Vec<Operand> {
    let mut operands = Vec::new();
    let mut iter = commands.into_iter().peekable();
    while let Some(command) = iter.next() {
        operands.push(match command {
            Command::Next => Operand::Next(1 + count_command(&mut iter, Command::Next)),
            Command::Prev => Operand::Prev(1 + count_command(&mut iter, Command::Prev)),
            Command::Increment => Operand::Increment(1 + count_command(&mut iter, Command::Increment) as u8),
            Command::Decrement => Operand::Decrement(1 + count_command(&mut iter, Command::Decrement) as u8),
            Command::Print => Operand::Print,
            Command::Read => Operand::Read,
            Command::LoopBegin => Operand::LoopBegin,
            Command::LoopEnd => Operand::LoopEnd,
        })
    }
    operands
}

fn main() {
    let commands = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    let mut machine = Machine::new(pass(parse(&commands)));
    machine.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_hello_world(b: &mut Bencher) {
        let commands = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
        b.iter(|| {
            let mut machine = Machine::new(pass(parse(&commands)));
            machine.run();
        })
    }
}
