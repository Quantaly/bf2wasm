pub mod parser;
pub mod optimizer;
pub mod wasm;

pub use parser::parse;
pub use optimizer::optimize;
pub use wasm::compile_wasm;

use std::io::Read;

/// Elements of Brainfuck's syntax.
#[derive(Debug, PartialEq)]
pub enum BrainfuckSyntax {
    /// Represents some number of contiguous `<` or `>` commands.
    MovePointer(i32),
    /// Represents some number of contiguous `+` or `-` commands.
    ModifyValue(i64),
    /// Represents a `.` command.
    Output,
    /// Represents a `,` command.
    Input,
    /// Represents a `[`, its matching `]`, and all commands in between.
    Loop(Vec<BrainfuckSyntax>),
    /// A "synthetic" piece of syntax with no counterpart in Brainfuck, used by the optimizer to simplify removing syntax.
    NoOp,
}

/// If this test fails, all the rest are pretty (brain)fucking useless.
#[test]
fn syntax_equality() {
    use BrainfuckSyntax::*;
    assert_eq!(
        vec![Input, Output, MovePointer(4), Loop(vec![Input])],
        vec![Input, Output, MovePointer(4), Loop(vec![Input])]
    );
    assert_ne!(vec![Output, Input], vec![Input, Output]);
}

/// Options that can be passed to the compiler, affecting the behavior
/// of the compiled program.
pub struct CompilerOptions {
    /// The minimum number of cells that the program should have access to.
    pub num_cells: u32,
    /// The size of each cell and the range of values they can store.
    pub cell_size: CellSize,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        CompilerOptions {
            num_cells: 32_768,
            cell_size: CellSize::I32,
        }
    }
}

/// The size of a cell.
#[derive(Debug)]
pub enum CellSize {
    /// Eight bits.
    I8,
    /// 16 bits.
    I16,
    /// 32 bits.
    I32,
    /// 64 bits.
    I64,
}

impl CellSize {
    pub fn byte_length(&self) -> u32 {
        match self {
            CellSize::I8 => 1,
            CellSize::I16 => 2,
            CellSize::I32 => 4,
            CellSize::I64 => 8,
        }
    }
}

pub fn parse_and_optimize(input: &mut impl Read) -> Result<Vec<BrainfuckSyntax>, parser::ParseError> {
    let mut ret = parse(input)?;
    optimize(&mut ret);
    Ok(ret)
}
