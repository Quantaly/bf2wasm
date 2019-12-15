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
        let mut result = inner_result.0;
        // Optimize away initial comment loops.
        let mut i = 0;
        while i < result.len()
            && match result[i] {
                Loop(_) => true,
                _ => false,
            }
        {
            i += 1;
        }
        if i == 0 {
            Ok(result)
        } else {
            let ret = result.split_off(i);
            Ok(ret)
        }
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
fn test_remove_initial_comment_loop() -> Result<(), ParseError> {
    use std::io::Cursor;
    let actual = parse(&mut Cursor::new(
        "[this loop is useless, it shouldn't be preserved.]++.",
    ))?;
    let expected = vec![ModifyValue(2), Output];
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

    let actual = parse(&mut Cursor::new("[this. is. stupid.]"))?;
    assert_eq!(&actual, &expected);

    Ok(())
}

const INC_POINTER: u8 = 0x3e; // '>'
const DEC_POINTER: u8 = 0x3c; // '<'
const INC_VALUE: u8 = 0x2b; // '+'
const DEC_VALUE: u8 = 0x2d; // '-'
const OUTPUT: u8 = 0x2e; // '.'
const INPUT: u8 = 0x2c; // ','
const BEGIN_LOOP: u8 = 0x5b; // '['
const END_LOOP: u8 = 0x5d; // ']'

#[test]
fn character_codes_correct() {
    let mut char_buf = [0u8; 1];

    '>'.encode_utf8(&mut char_buf);
    assert_eq!(char_buf[0], INC_POINTER);

    '<'.encode_utf8(&mut char_buf);
    assert_eq!(char_buf[0], DEC_POINTER);

    '+'.encode_utf8(&mut char_buf);
    assert_eq!(char_buf[0], INC_VALUE);

    '-'.encode_utf8(&mut char_buf);
    assert_eq!(char_buf[0], DEC_VALUE);

    '.'.encode_utf8(&mut char_buf);
    assert_eq!(char_buf[0], OUTPUT);

    ','.encode_utf8(&mut char_buf);
    assert_eq!(char_buf[0], INPUT);

    '['.encode_utf8(&mut char_buf);
    assert_eq!(char_buf[0], BEGIN_LOOP);

    ']'.encode_utf8(&mut char_buf);
    assert_eq!(char_buf[0], END_LOOP);
}

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
