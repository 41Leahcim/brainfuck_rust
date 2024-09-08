extern crate alloc;

use alloc::collections::VecDeque;
use std::io::{self, Read, Write};

use crate::command::Command;

#[derive(Debug)]
pub struct Program(Vec<Command>);

impl From<Vec<Command>> for Program {
    fn from(value: Vec<Command>) -> Self {
        // Create a program from the command buffer
        let result = Self(value);

        // Make sure all loops are opened and closed
        result.check();
        result
    }
}

impl Program {
    fn check(&self) {
        let mut active_loops = 0_usize;

        // Iterate through the commands
        for command in &self.0 {
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

    pub fn execute(&self) {
        // Create a program counter, pointer and data buffer
        let mut pc = 0;
        let mut pointer = 0;
        let mut data = VecDeque::<u8>::new();
        data.push_back(0);

        // Iterate through the commands
        while let Some(command) = self.0.get(pc) {
            // Execute the command
            match command {
                Command::DecrementPointer => {
                    if pointer > 0 {
                        pointer -= 1;
                    } else {
                        data.push_front(0);
                    }
                }
                Command::IncrementPointer => {
                    pointer += 1;
                    if pointer >= data.len() {
                        data.push_back(0);
                    }
                }
                Command::DecrementValue => data[pointer] -= 1,
                Command::IncrementValue => data[pointer] += 1,
                Command::Input => {
                    data[pointer] = io::stdin()
                        .bytes()
                        .next()
                        .expect("Failed to read input")
                        .expect("Failed to read input");
                }
                Command::Output => {
                    while io::stdout()
                        .write(&[data[pointer]])
                        .expect("Failed to print data")
                        == 0
                    {}
                }
                Command::StartOfLoop if data[pointer] == 0 => {
                    // Go to the end of the loop
                    let mut active_loops = 1;
                    while active_loops > 0 {
                        pc += 1;
                        match self.0[pc] {
                            Command::StartOfLoop => active_loops += 1,
                            Command::EndOfLoop => active_loops -= 1,
                            Command::IncrementPointer
                            | Command::DecrementPointer
                            | Command::IncrementValue
                            | Command::DecrementValue
                            | Command::Output
                            | Command::Input => {}
                        }
                    }
                }
                Command::EndOfLoop if data[pointer] != 0 => {
                    // Go to the beginning of the loop
                    let mut ended_loops = 1;
                    while ended_loops > 0 {
                        pc -= 1;
                        match self.0[pc] {
                            Command::StartOfLoop => ended_loops -= 1,
                            Command::EndOfLoop => ended_loops += 1,
                            Command::IncrementPointer
                            | Command::DecrementPointer
                            | Command::IncrementValue
                            | Command::DecrementValue
                            | Command::Output
                            | Command::Input => {}
                        }
                    }
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
