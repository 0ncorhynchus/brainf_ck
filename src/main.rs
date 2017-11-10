use std::io::{self, Read};

struct Machine {
    pointer: usize,
    memory: Vec<u8>,
    commands: Vec<char>,
    command_index: usize,
}

impl Machine {
    const INITIAL_SIZE: usize = 300000;

    fn new(commands: String) -> Self {
        Machine {
            pointer: 0,
            memory: vec![0; Machine::INITIAL_SIZE],
            commands: commands.chars().collect(),
            command_index: 0,
        }
    }

    fn get_value(&self) -> u8 {
        self.memory[self.pointer]
    }

    fn get_command(&self) -> char {
        self.commands[self.command_index]
    }

    fn is_finished(&self) -> bool {
        self.command_index < self.commands.len()
    }

    fn run(&mut self) {
        while self.is_finished() {
            match self.get_command() {
                '>' => self.pointer += 1,
                '<' => self.pointer -= 1,
                '+' => self.memory[self.pointer] += 1,
                '-' => self.memory[self.pointer] -= 1,
                '.' => {
                    print!("{}", char::from(self.get_value()));
                },
                ',' => {
                    let stdin = io::stdin();
                    let mut handle = stdin.lock();
                    handle.read(&mut self.memory[self.pointer..self.pointer+1]).unwrap();
                },
                '[' => {
                    if self.get_value() == 0 {
                        while self.get_command() != ']' {
                            self.command_index += 1;
                        }
                    }
                },
                ']' => {
                    if self.get_value() != 0 {
                        let mut num_inner_loops = 1;
                        while num_inner_loops != 0 {
                            self.command_index -= 1;
                            match self.get_command() {
                                '[' => {
                                    num_inner_loops -= 1;
                                },
                                ']' => {
                                    num_inner_loops += 1;
                                },
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }
            self.command_index += 1;
        }
    }
}

fn main() {
    let commands = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    let mut machine = Machine::new(commands);
    machine.run();
}
