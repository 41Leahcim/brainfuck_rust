#![warn(clippy::pedantic, clippy::nursery, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::blanket_clippy_restriction_lints,
    clippy::print_stderr,
    clippy::expect_used,
    clippy::implicit_return,
    clippy::missing_trait_methods,
    clippy::arithmetic_side_effects,
    clippy::default_numeric_fallback,
    clippy::shadow_reuse,
    clippy::indexing_slicing,
    clippy::pattern_type_mismatch,
    clippy::separated_literal_suffix,
    clippy::match_on_vec_items,
    clippy::single_call_fn,
    clippy::wildcard_enum_match_arm,
    clippy::use_debug,
    clippy::min_ident_chars,
    reason = ""
)]

use std::{
    env::args,
    fs::File,
    io::{BufReader, Read},
};

use chars::Chars;
use command::Command;
use optimized_command::OptimizedCommand;
use optimized_program::OptimizedProgram;
use program::Program;

mod chars;
mod command;
mod optimized_command;
mod optimized_program;
mod program;

/// Compiles the program without optimizations.
fn unoptimized_compiler<Code: Iterator<Item = Command>>(commands: Code) -> Program {
    // Collect the commands into a buffer and store it as a program, if valid
    Program::from(commands.collect::<Vec<_>>())
}

/// Compiles the program, an optimized program may not always work as expected
fn optimized_compiler<Code: Iterator<Item = Command>>(commands: Code) -> OptimizedProgram {
    // Turn the commands into optimized commands and store it in a program, if valid
    OptimizedProgram::from(OptimizedCommand::optimize_commands(commands))
}

fn main() {
    // Read the name of the brainfuck file and check whether the code should be optimized
    let mut file_name = None;
    let mut optimization = false;
    for argument in args().skip(1).take(2) {
        match argument.as_str() {
            "-O" => optimization = true,
            _ => file_name = Some(argument),
        }
    }
    let file_name = file_name.expect("No filename found");

    // Read the brainfuck file
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Create an iterator to read the commands
    let commands = Chars::from(reader.bytes().map_while(Result::ok))
        .map(Command::try_from)
        .filter_map(Result::ok);

    // Compile and run the program, only optimize if requested
    if optimization {
        optimized_compiler(commands).execute();
    } else {
        unoptimized_compiler(commands).execute();
    }

    eprintln!();
}
