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
    Loop(Vec<Operand>),
}

struct Machine {
    pointer: usize,
    memory: Vec<u8>,
}

impl Machine {
    const INITIAL_SIZE: usize = 300000;

    fn new() -> Self {
        Machine {
            pointer: 0,
            memory: vec![0; Machine::INITIAL_SIZE],
        }
    }

    fn get_value(&self) -> u8 {
        self.memory[self.pointer]
    }

    fn exec(&mut self, operand: &Operand) {
        match operand {
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
            &Operand::Loop(ref operands) => {
                while self.get_value() != 0 {
                    for op in operands.iter() {
                        self.exec(op);
                    }
                }
            },
        }
    }

    fn run(&mut self, operands: &Vec<Operand>) {
        for operand in operands {
            self.exec(operand);
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
        if iter.peek() != Some(&target) {
            break;
        }
        count += 1;
        iter.next();
    }
    count
}

fn pass<I>(input: &mut Peekable<I>, depth: usize) -> Vec<Operand>
where I: Iterator<Item=Command>
{
    let mut operands = Vec::new();
    while let Some(command) = input.next() {
        operands.push(match command {
            Command::Next      => Operand::Next(1 + count_command(input, Command::Next)),
            Command::Prev      => Operand::Prev(1 + count_command(input, Command::Prev)),
            Command::Increment => Operand::Increment(1 + count_command(input, Command::Increment) as u8),
            Command::Decrement => Operand::Decrement(1 + count_command(input, Command::Decrement) as u8),
            Command::Print     => Operand::Print,
            Command::Read      => Operand::Read,
            Command::LoopBegin => Operand::Loop(pass(input, depth+1)),
            Command::LoopEnd   => { break; },
        })
    }
    operands
}

fn compile(commands: Vec<Command>) -> Vec<Operand> {
    let mut iter = commands.into_iter().peekable();
    pass(&mut iter, 0)
}

fn main() {
    let commands = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    let operands = compile(parse(&commands));
    let mut machine = Machine::new();
    machine.run(&operands);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_hello_world(b: &mut Bencher) {
        let commands = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
        let operands = compile(parse(&commands));
        b.iter(|| {
            let mut machine = Machine::new();
            machine.run(&operands);
        })
    }
}
