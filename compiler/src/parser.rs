use super::BrainfuckSyntax;
use super::BrainfuckSyntax::*;
use std::convert::From;
use std::io::prelude::*;
use std::io::Error;

/// Parses a Brainfuck program read from [input].
pub fn parse(input: &mut impl Read) -> Result<Vec<BrainfuckSyntax>, ParseError> {
    let inner_result = _parse(input)?;
    if !inner_result.1 {
        // should absolutely EOF here
        Err(ParseError::SyntaxError(String::from("unbalanced brackets")))
    } else {
        Ok(inner_result.0)
    }
}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError(String),
    IoError(Error),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(
            &(match self {
                ParseError::SyntaxError(msg) => format!("syntax error: {}", msg),
                ParseError::IoError(err) => format!("IO error: {}", err),
            })[..],
        )?;
        Ok(())
    }
}

impl From<Error> for ParseError {
    fn from(err: Error) -> ParseError {
        ParseError::IoError(err)
    }
}

#[test]
fn test_parsing() -> Result<(), ParseError> {
    use std::io::Cursor;
    let actual = parse(&mut Cursor::new(",.>+++++[.-]<."))?;
    let expected = vec![
        Input,
        Output,
        MovePointer(1),
        ModifyValue(5),
        Loop(vec![Output, ModifyValue(-1)]),
        MovePointer(-1),
        Output,
    ];
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn test_parsing_errors() {
    use std::io::Cursor;
    if let Err(err) = parse(&mut Cursor::new("[")) {
        match err {
            ParseError::SyntaxError(msg) => {
                if !msg.contains("unbalanced brackets") {
                    panic!("wrong error message for unbalanced open bracket: {}", msg);
                }
            }
            _ => panic!(format!(
                "wrong kind of error for unbalanced open bracket: {:?}",
                err
            )),
        }
    } else {
        panic!("unbalanced open bracket didn't error");
    }

    if let Err(err) = parse(&mut Cursor::new("]")) {
        match err {
            ParseError::SyntaxError(msg) => {
                if !msg.contains("unbalanced brackets") {
                    panic!(
                        "wrong error message for unbalanced closing bracket: {}",
                        msg
                    );
                }
            }
            _ => panic!(format!(
                "wrong kind of error for unbalanced closing bracket: {:?}",
                err
            )),
        }
    } else {
        panic!("unbalanced closing bracket didn't error");
    }
}

#[test]
fn test_parsing_weird_chars() -> Result<(), ParseError> {
    use std::io::Cursor;
    let actual = parse(&mut Cursor::new("Hey, what's 9 + 10? 21. ;)"))?;
    let expected = vec![Input, ModifyValue(1), Output];
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn test_end_with_compound_instruction() -> Result<(), ParseError> {
    use std::io::Cursor;

    let actual = parse(&mut Cursor::new(">>.>>>"))?;
    let expected = vec![MovePointer(2), Output, MovePointer(3)];
    assert_eq!(expected, actual);

    let actual = parse(&mut Cursor::new("++.+++"))?;
    let expected = vec![ModifyValue(2), Output, ModifyValue(3)];
    assert_eq!(expected, actual);

    let actual = parse(&mut Cursor::new(",-[+>,-]"))?;
    let expected = vec![
        Input,
        ModifyValue(-1),
        Loop(vec![ModifyValue(1), MovePointer(1), Input, ModifyValue(-1)]),
    ];
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn test_empty() -> Result<(), ParseError> {
    use std::io::Cursor;
    let expected = vec![];

    let actual = parse(&mut Cursor::new(""))?;
    assert_eq!(&actual, &expected);

    let actual = parse(&mut Cursor::new("hewwo"))?;
    assert_eq!(&actual, &expected);

    Ok(())
}

const INC_POINTER: u8 = b'>';
const DEC_POINTER: u8 = b'<';
const INC_VALUE: u8 = b'+';
const DEC_VALUE: u8 = b'-';
const OUTPUT: u8 = b'.';
const INPUT: u8 = b',';
const BEGIN_LOOP: u8 = b'[';
const END_LOOP: u8 = b']';

/// The inner parsing logic for a Brainfuck program.
///
/// It should emit appropriate [BrainfuckSyntax] values for all syntax characters in the input string,
/// calling itself recursively to handle loops. The [bool] returned in the tuple should be `true` iff
/// the function returned because there were no characters left to read from the input; it should otherwise
/// return if it encounters a closing bracket.
fn _parse(input: &mut impl Read) -> Result<(Vec<BrainfuckSyntax>, bool), ParseError> {
    let mut ret = Vec::new();
    let mut current_instr: Option<BrainfuckSyntax> = None;
    let mut current_byte = [0u8; 1];
    loop {
        let bytes_read = input.read(&mut current_byte)?;
        if bytes_read == 0 {
            if let Some(instr) = current_instr.take() {
                // this particular edge case wouldn't actually change any observable behavior.
                // after all, if the last thing the program does isn't I/O, there's no way to know it did it.
                // but, for completeness's sake, removing this can be left to the optimizer.
                // besides, maybe in the future the memory will be imported/exported, and then this will be observable.
                ret.push(instr);
            }
            return Ok((ret, true));
        }
        match current_byte[0] {
            INC_POINTER => match &current_instr {
                Some(instr) => match instr {
                    MovePointer(value) => current_instr = Some(MovePointer(value + 1)),
                    _ => {
                        ret.push(current_instr.take().unwrap());
                        current_instr = Some(MovePointer(1));
                    }
                },
                None => current_instr = Some(MovePointer(1)),
            },
            DEC_POINTER => match &current_instr {
                Some(instr) => match instr {
                    MovePointer(value) => current_instr = Some(MovePointer(value - 1)),
                    _ => {
                        ret.push(current_instr.take().unwrap());
                        current_instr = Some(MovePointer(-1));
                    }
                },
                None => current_instr = Some(MovePointer(-1)),
            },
            INC_VALUE => match &current_instr {
                Some(instr) => match instr {
                    ModifyValue(value) => current_instr = Some(ModifyValue(value + 1)),
                    _ => {
                        ret.push(current_instr.take().unwrap());
                        current_instr = Some(ModifyValue(1));
                    }
                },
                None => current_instr = Some(ModifyValue(1)),
            },
            DEC_VALUE => match &current_instr {
                Some(instr) => match instr {
                    ModifyValue(value) => current_instr = Some(ModifyValue(value - 1)),
                    _ => {
                        ret.push(current_instr.take().unwrap());
                        current_instr = Some(ModifyValue(-1));
                    }
                },
                None => current_instr = Some(ModifyValue(-1)),
            },
            OUTPUT => {
                if let Some(instr) = current_instr.take() {
                    ret.push(instr);
                }
                ret.push(Output);
            }
            INPUT => {
                if let Some(instr) = current_instr.take() {
                    ret.push(instr);
                }
                ret.push(Input);
            }
            BEGIN_LOOP => {
                if let Some(instr) = current_instr.take() {
                    ret.push(instr);
                }
                let inner_result = _parse(input)?;
                if inner_result.1 {
                    // should not EOF here
                    return Err(ParseError::SyntaxError(String::from("unbalanced brackets")));
                };
                ret.push(Loop(inner_result.0))
            }
            END_LOOP => {
                if let Some(instr) = current_instr.take() {
                    ret.push(instr);
                }
                return Ok((ret, false));
            }
            _ => (),
        }
    }
}
