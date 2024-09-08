use crate::command::Command;

/// A command type that can represent multiple brainfuck commands with 1 command.
/// It also stores the start and end of each loop as indices to make jumps faster.
/// This can save calculations on execution, but doesn't work correctly for all programs yet.
#[derive(Debug, PartialEq, Eq)]
pub enum OptimizedCommand {
    AddPointer(usize),
    SubtractPointer(usize),
    AddValue(u8),
    SubtractValue(u8),
    Output,
    Input,
    StartOfLoop { end: usize },
    EndOfLoop { start: usize },
}

impl OptimizedCommand {
    fn optimize_increment_pointer(
        current_command: Option<Self>,
        optimized_program: &mut Vec<Self>,
    ) -> Option<Self> {
        match current_command {
            None => Some(Self::AddPointer(1)),
            Some(Self::AddPointer(value)) => {
                (value < usize::MAX).then(|| Self::AddPointer(value + 1))
            }
            Some(Self::SubtractPointer(value)) => {
                (value > 1).then(|| Self::SubtractPointer(value - 1))
            }
            Some(optimized_command) => {
                optimized_program.push(optimized_command);
                Some(Self::AddPointer(1))
            }
        }
    }

    fn optimize_decrement_pointer(
        current_command: Option<Self>,
        optimized_program: &mut Vec<Self>,
    ) -> Option<Self> {
        match current_command {
            None => Some(Self::SubtractPointer(1)),
            Some(Self::AddPointer(value)) => (value > 1).then(|| Self::AddPointer(value - 1)),
            Some(Self::SubtractPointer(value)) => {
                (value < usize::MAX).then(|| Self::SubtractPointer(value + 1))
            }
            Some(optimized_command) => {
                optimized_program.push(optimized_command);
                Some(Self::SubtractPointer(1))
            }
        }
    }

    fn optimize_increment_value(
        current_command: Option<Self>,
        optimized_program: &mut Vec<Self>,
    ) -> Option<Self> {
        match current_command {
            None => Some(Self::AddValue(1)),
            Some(Self::AddValue(value)) => (value < u8::MAX).then(|| Self::AddValue(value + 1)),
            Some(Self::SubtractValue(value)) => (value > 1).then(|| Self::SubtractValue(value - 1)),
            Some(optimized_command) => {
                optimized_program.push(optimized_command);
                Some(Self::AddValue(1))
            }
        }
    }

    fn optimize_decrement_value(
        current_command: Option<Self>,
        optimized_program: &mut Vec<Self>,
    ) -> Option<Self> {
        match current_command {
            None => Some(Self::SubtractValue(1)),
            Some(Self::AddValue(value)) => (value > 1).then(|| Self::AddValue(value - 1)),
            Some(Self::SubtractValue(value)) => {
                (value < u8::MAX).then(|| Self::SubtractValue(value + 1))
            }
            Some(optimized_command) => {
                optimized_program.push(optimized_command);
                Some(Self::SubtractValue(1))
            }
        }
    }

    fn optimize_output(current_command: Option<Self>, optimized_program: &mut Vec<Self>) -> Self {
        if let Some(optimized_command) = current_command {
            optimized_program.push(optimized_command);
        }
        Self::Output
    }

    fn optimize_input(current_command: Option<Self>, optimized_program: &mut Vec<Self>) -> Self {
        if let Some(optimized_command) = current_command {
            optimized_program.push(optimized_command);
        }
        Self::Input
    }

    fn optimize_start_of_loop(
        current_command: Option<Self>,
        optimized_program: &mut Vec<Self>,
    ) -> Self {
        if let Some(optimized_command) = current_command {
            optimized_program.push(optimized_command);
        }
        Self::StartOfLoop { end: 0 }
    }

    fn optimize_end_of_loop(
        current_command: Option<Self>,
        optimized_program: &mut Vec<Self>,
    ) -> Self {
        if let Some(optimized_command) = current_command {
            optimized_program.push(optimized_command);
        }

        // Store the current program length
        let current_program_length = optimized_program.len();

        // Find the beginning of the loop
        let mut closed_loops = 1;
        let (index, start_of_loop) = optimized_program
            .iter_mut()
            .enumerate()
            .rev()
            .find(|(_, optimized_command)| {
                match optimized_command {
                    Self::EndOfLoop { .. } => closed_loops += 1,
                    Self::StartOfLoop { .. } => closed_loops -= 1,
                    _ => {}
                }
                closed_loops == 0
            })
            .expect("Unmatched end of loop");

        // Make sure the found loop start is still unmatched
        assert_eq!(
            start_of_loop,
            &mut Self::StartOfLoop { end: 0 },
            "Found a wrong start of loop while looking for a match for end of loop"
        );

        // Store the current program length as the end of the loop,
        // as that will be the index of the end of the loop
        *start_of_loop = Self::StartOfLoop {
            end: current_program_length,
        };
        Self::EndOfLoop { start: index }
    }

    pub fn optimize_commands<Commands: Iterator<Item = Command>>(commands: Commands) -> Vec<Self> {
        // Create a variable to store the current optimized command and a buffer for the full program
        let mut current_command = None;
        let mut optimized_program = Vec::new();

        // Iterate through the commands
        for command in commands {
            // Store the new command, pushing the command on change of command type.
            // Operations on pointers are the same type.
            // Operations on values are the same type.
            // Everything else is their own type
            current_command = match command {
                Command::IncrementPointer => {
                    Self::optimize_increment_pointer(current_command, &mut optimized_program)
                }
                Command::DecrementPointer => {
                    Self::optimize_decrement_pointer(current_command, &mut optimized_program)
                }
                Command::IncrementValue => {
                    Self::optimize_increment_value(current_command, &mut optimized_program)
                }
                Command::DecrementValue => {
                    Self::optimize_decrement_value(current_command, &mut optimized_program)
                }
                Command::Output => Some(Self::optimize_output(
                    current_command,
                    &mut optimized_program,
                )),
                Command::Input => Some(Self::optimize_input(
                    current_command,
                    &mut optimized_program,
                )),
                Command::StartOfLoop => Some(Self::optimize_start_of_loop(
                    current_command,
                    &mut optimized_program,
                )),
                Command::EndOfLoop => Some(Self::optimize_end_of_loop(
                    current_command,
                    &mut optimized_program,
                )),
            };
        }

        // Make sure all commands are stored in the program
        if let Some(command) = current_command {
            optimized_program.push(command);
        }
        optimized_program
    }
}

#[cfg(test)]
mod tests{
    use crate::command::Command;

    use super::OptimizedCommand;

    #[test]
    fn no_duplicates(){
        let program = [Command::StartOfLoop, Command::IncrementPointer, Command::IncrementValue, Command::DecrementPointer, Command::DecrementValue, Command::EndOfLoop];
        let optimized_program = OptimizedCommand::optimize_commands(program.into_iter());
        assert_eq!(optimized_program, [OptimizedCommand::StartOfLoop { end: 5 }, OptimizedCommand::AddPointer(1), OptimizedCommand::AddValue(1), OptimizedCommand::SubtractPointer(1), OptimizedCommand::SubtractValue(1), OptimizedCommand::EndOfLoop { start: 0 }]);
    }
    
    #[test]
    fn empty_loop(){
        let program = [Command::StartOfLoop, Command::IncrementPointer, Command::DecrementPointer, Command::IncrementValue, Command::DecrementValue, Command::EndOfLoop];
        let optimized_program = OptimizedCommand::optimize_commands(program.into_iter());
        assert_eq!(optimized_program, [OptimizedCommand::StartOfLoop { end: 1 }, OptimizedCommand::EndOfLoop { start: 0 }]);
    }

    #[test]
    fn add_2(){
        let program = [Command::StartOfLoop, Command::IncrementPointer, Command::IncrementPointer, Command::IncrementValue, Command::IncrementValue, Command::DecrementPointer, Command::DecrementPointer, Command::DecrementValue, Command::DecrementValue, Command::EndOfLoop];
        let optimized_program = OptimizedCommand::optimize_commands(program.into_iter());
        assert_eq!(optimized_program, [OptimizedCommand::StartOfLoop { end: 5 }, OptimizedCommand::AddPointer(2), OptimizedCommand::AddValue(2), OptimizedCommand::SubtractPointer(2), OptimizedCommand::SubtractValue(2), OptimizedCommand::EndOfLoop { start: 0 }]);
    }
}
