extern crate alloc;

use alloc::collections::VecDeque;
use std::io::{self, Read, Write};

use crate::command::Command;

#[derive(Debug)]
pub struct Program {
    commands: Vec<Command>,
    data: VecDeque<u8>,
}

impl From<Vec<Command>> for Program {
    fn from(value: Vec<Command>) -> Self {
        // Create a program from the command buffer
        let result = Self {
            commands: value,
            data: VecDeque::new(),
        };

        // Make sure all loops are opened and closed
        result.check();
        result
    }
}

impl Program {
    fn check(&self) {
        let mut active_loops = 0_usize;

        // Iterate through the commands
        for command in &self.commands {
            // Increment the number of active loops on the start of loops.
            // Decrement the number of active loops on the end of loops.
            // Panic if the end of a loop is found, while there are no active loops.
            match command {
                Command::StartOfLoop => active_loops += 1,
                Command::EndOfLoop => {
                    assert_ne!(active_loops, 0, "Unexpected end of loop");
                    active_loops -= 1;
                }
                Command::IncrementPointer
                | Command::DecrementPointer
                | Command::IncrementValue
                | Command::DecrementValue
                | Command::Output
                | Command::Input => {}
            }
        }

        // Panic if not all loops were closed
        assert_eq!(active_loops, 0, "Missing end of loop");
    }

    fn decrement_pointer(&mut self, pointer: &mut usize) {
        if *pointer > 0 {
            *pointer -= 1;
        } else {
            self.data.push_front(0);
        }
    }

    fn increment_pointer(&mut self, pointer: &mut usize) {
        *pointer += 1;
        if *pointer >= self.data.len() {
            self.data.push_back(0);
        }
    }

    fn input(&mut self, pointer: usize) {
        self.data[pointer] = io::stdin()
            .bytes()
            .next()
            .expect("Failed to read input")
            .expect("Failed to read input");
    }

    fn output(&self, pointer: usize) {
        while io::stdout()
            .write(&[self.data[pointer]])
            .expect("Failed to print data")
            == 0
        {}
    }

    #[expect(clippy::unwrap_used, reason = "Every loop has a valid start and end")]
    fn start_of_loop(&self, pc: &mut usize) {
        // Go to the end of the loop
        let mut active_loops = 1;
        *pc = self
            .commands
            .iter()
            .enumerate()
            .skip(*pc + 1)
            .find(|(_, command)| {
                match command {
                    Command::StartOfLoop => active_loops += 1,
                    Command::EndOfLoop => active_loops -= 1,
                    _ => {}
                }
                active_loops == 0
            })
            .unwrap()
            .0;
    }

    #[expect(clippy::unwrap_used, reason = "Every loop has a valid start and end")]
    fn end_of_loop(&self, pc: &mut usize) {
        // Go to the beginning of the loop
        let mut ended_loops = 1;
        *pc = self.commands[..*pc]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, command)| {
                match command {
                    Command::StartOfLoop => ended_loops -= 1,
                    Command::EndOfLoop => ended_loops += 1,
                    _ => {}
                }
                ended_loops == 0
            })
            .unwrap()
            .0;
    }

    pub fn execute(&mut self) {
        // Create a program counter, pointer and data buffer
        let mut pc = 0;
        let mut pointer = 0;
        self.data.clear();
        self.data.push_back(0);

        // Iterate through the commands
        while let Some(command) = self.commands.get(pc) {
            // Execute the command
            match command {
                Command::DecrementPointer => self.decrement_pointer(&mut pointer),
                Command::IncrementPointer => self.increment_pointer(&mut pointer),
                Command::DecrementValue => self.data[pointer] -= 1,
                Command::IncrementValue => self.data[pointer] += 1,
                Command::Input => self.input(pointer),
                Command::Output => self.output(pointer),
                Command::StartOfLoop if self.data[pointer] == 0 => {
                    self.start_of_loop(&mut pc);
                }
                Command::EndOfLoop if self.data[pointer] != 0 => {
                    self.end_of_loop(&mut pc);
                }
                Command::StartOfLoop | Command::EndOfLoop => {}
            }

            // Increment the program counter
            pc += 1;
        }

        // Flush the output
        #[expect(clippy::unwrap_used, reason = "Flushing stdout shouldn't go wrong.")]
        io::stdout().flush().unwrap();
    }
}
