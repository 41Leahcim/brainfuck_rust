extern crate alloc;

use alloc::collections::VecDeque;
use std::io::{self, Read, Write};

use crate::optimized_command::OptimizedCommand;

#[derive(Debug)]
pub struct OptimizedProgram{
    program: Vec<OptimizedCommand>,
    data: VecDeque<u8>
}

impl From<Vec<OptimizedCommand>> for OptimizedProgram {
    fn from(value: Vec<OptimizedCommand>) -> Self {
        // Create an optimized program
        let result = Self{program: value, data: VecDeque::new()};

        // Make sure all loops are opened AND closed
        result.check();
        result
    }
}

impl OptimizedProgram {
    fn check(&self) {
        let mut active_loops = 0_usize;

        // iterate though the commands
        for command in &self.program {
            // Increment the number of active loops on the start of loops.
            // Decrement the number of active loops on the end of loops.
            // Panic when the end of a loop is found while the number of active loops is 0.
            match command {
                OptimizedCommand::StartOfLoop { .. } => active_loops += 1,
                OptimizedCommand::EndOfLoop { .. } => {
                    assert_ne!(active_loops, 0, "Unexpected end of loop");
                    active_loops -= 1;
                }
                _ => {}
            }
        }

        // Panic if there are loops that haven't been closed
        assert_eq!(active_loops, 0, "Missing end of loop");
    }

    pub fn execute(&mut self) {
        // Create a program counter, pointer, and data buffer
        let mut pc = 0;
        let mut pointer = 0;
        self.data.push_back(0);

        // Iterate through the commands
        while let Some(command) = self.program.get(pc) {
            // Execute the current command
            match command {
                OptimizedCommand::SubtractPointer(value) => {
                    if pointer >= *value {
                        pointer -= *value;
                    } else {
                        for _ in pointer..=*value {
                            self.data.push_front(0);
                        }
                    }
                }
                OptimizedCommand::AddPointer(value) => {
                    pointer += value;
                    for _ in self.data.len()..=pointer {
                        self.data.push_back(0);
                    }
                }
                OptimizedCommand::SubtractValue(value) => self.data[pointer] -= value,
                OptimizedCommand::AddValue(value) => self.data[pointer] += value,
                OptimizedCommand::Input => {
                    self.data[pointer] = io::stdin()
                        .bytes()
                        .next()
                        .expect("Failed to read input")
                        .expect("Failed to read input");
                }
                OptimizedCommand::Output => {
                    while io::stdout()
                        .write(&[self.data[pointer]])
                        .expect("Failed to print data")
                        == 0
                    {}
                }
                OptimizedCommand::StartOfLoop { end } if self.data[pointer] == 0 => {
                    pc = *end;
                }
                OptimizedCommand::EndOfLoop { start } if self.data[pointer] != 0 => {
                    pc = *start;
                }
                OptimizedCommand::StartOfLoop { .. } | OptimizedCommand::EndOfLoop { .. } => {}
            }

            // Continue to the next command or end of the program
            pc += 1;
        }

        // Flush the output
        #[expect(clippy::unwrap_used, reason = "Flushing stdout shouldn't go wrong.")]
        io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
mod tests{
    use crate::optimized_command::OptimizedCommand;

    use super::OptimizedProgram;

    #[test]
    fn adding_two(){
        let mut program = OptimizedProgram::from(vec![OptimizedCommand::AddValue(2)]);
        program.execute();
        assert_eq!(program.data, [2]);
    }

    #[test]
    fn moving_data(){
        let mut program = OptimizedProgram::from(vec![OptimizedCommand::AddValue(2), OptimizedCommand::StartOfLoop { end: 6 }, OptimizedCommand::AddPointer(1), OptimizedCommand::AddValue(1), OptimizedCommand::SubtractPointer(1), OptimizedCommand::SubtractValue(1), OptimizedCommand::EndOfLoop { start: 1 }]);
        program.execute();
        assert_eq!(program.data, [0, 2]);
    }
}
