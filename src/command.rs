use core::fmt::{self, Display, Formatter};
use std::error;

/// Contains a invalid brainfuck command (will just be ignored)
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct InvalidCommand(char);

impl error::Error for InvalidCommand {}

impl Display for InvalidCommand {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "Invalid command: {}", self.0)
    }
}

#[derive(Debug)]
pub enum Command {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    StartOfLoop,
    EndOfLoop,
}

impl TryFrom<char> for Command {
    type Error = InvalidCommand;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        // Map the character to the correct Command
        match value {
            '>' => Ok(Self::IncrementPointer),
            '<' => Ok(Self::DecrementPointer),
            '+' => Ok(Self::IncrementValue),
            '-' => Ok(Self::DecrementValue),
            '.' => Ok(Self::Output),
            ',' => Ok(Self::Input),
            '[' => Ok(Self::StartOfLoop),
            ']' => Ok(Self::EndOfLoop),
            _ => Err(InvalidCommand(value)),
        }
    }
}
