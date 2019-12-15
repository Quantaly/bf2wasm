pub mod parser;
pub mod wasm;

pub use parser::parse;
pub use wasm::compile_wasm;

/// Elements of Brainfuck's syntax.
#[derive(Debug, PartialEq)]
pub enum BrainfuckSyntax {
    /// Represents some number of contiguous `<` or `>` commands.
    MovePointer(i32),
    /// Represents some number of contiguous `+` or `-` commands.
    ModifyValue(i32),
    /// Represents a `.` command.
    Output,
    /// Represents a `,` command.
    Input,
    /// Represents a `[`, its matching `]`, and all commands in between.
    Loop(Vec<BrainfuckSyntax>),
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
