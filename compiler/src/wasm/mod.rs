mod instr;

use super::BrainfuckSyntax;
use super::BrainfuckSyntax::*;
use instr::*;
use leb128;
use std::convert::TryInto;
use std::io;
use std::io::prelude::*;

const WASM_PRELUDE: &'static [u8] = &[
    0x00, 0x61, 0x73, 0x6d, // magic
    0x01, 0x00, 0x00, 0x00, // version
    /* type section */
    0x01, // id
    0x0c, // byte length
    0x03, // vec length
    0x60, 0x00, 0x01, 0x7f, // type 0: () -> (i32)
    0x60, 0x01, 0x7f, 0x00, // type 1: (i32) -> ()
    0x60, 0x00, 0x00, // type 2: () -> ()
    /* import section */
    0x02, // id
    0x22, // byte length
    0x02, // vec length
    // import func 0: io.get_value() -> (i32)
    0x02, 0x69, 0x6f, // mod: `io`
    0x0a, 0x72, 0x65, 0x61, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, // nm: `read_value`
    0x00, 0x00, // func() -> (i32) [type 0]
    // import func 1: io.put_value(i32) -> ()
    0x02, 0x69, 0x6f, // mod: `io`
    0x0b, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x75,
    0x65, // nm: `write_value`
    0x00, 0x01, // func(i32) -> () [type 1]
    /* function section */
    0x03, // id
    0x02, // byte length
    0x01, // vec length
    // func 2: () -> ()
    0x02, // type 2
    /* memory section */
    0x05, // id
    0x04, // byte length
    0x01, // vec length
    // memory 0
    0x01, 0x01, 0x01, // min 1, max 1
    /* start section */
    0x08, // id
    0x01, // byte length
    0x02, // function 2
];

pub fn compile_wasm(ast: Vec<BrainfuckSyntax>, output: &mut impl Write) -> io::Result<()> {
    output.write_all(WASM_PRELUDE)?;

    let mut func = vec![
        0x01, // locals vector length
        0x01, 0x7f, // one local of type i32
    ];
    emit_func_body(ast, &mut func)?;
    func.push(end()[0]);
    let func_len = u32_leb128(func.len().try_into().unwrap());

    let section_len = u32_leb128((func.len() + func_len.len() + 1).try_into().unwrap());

    output.write_all(&[0x0a])?; // code section id
    output.write_all(&section_len)?; // code section length
    output.write_all(&[0x01])?; // function vector length
    output.write_all(&func_len)?; // function 2 byte length (0 and 1 are the imports)
    output.write_all(&func)?; // function 2 contents

    Ok(())
}

fn emit_func_body(ast: Vec<BrainfuckSyntax>, output: &mut impl Write) -> io::Result<()> {
    for syntax in ast {
        match syntax {
            MovePointer(value) => {
                output.write_all(&local_get(0))?;
                output.write_all(&i32_const(value * 4))?; // stay i32-aligned
                output.write_all(&i32_add())?;
                output.write_all(&local_set(0))?;
            }
            ModifyValue(value) => {
                output.write_all(&local_get(0))?;
                output.write_all(&local_get(0))?;
                output.write_all(&i32_load(2, 0))?;
                output.write_all(&i32_const(value))?;
                output.write_all(&i32_add())?;
                output.write_all(&i32_store(2, 0))?;
            }
            Output => {
                output.write_all(&local_get(0))?;
                output.write_all(&i32_load(2, 0))?;
                output.write_all(&call(1))?; // <io.write_value>
            }
            Input => {
                output.write_all(&local_get(0))?;
                output.write_all(&call(0))?; // <io.read_value>
                output.write_all(&i32_store(2, 0))?;
            }
            Loop(contents) => {
                output.write_all(&wasm_loop(BlockType::Void))?;
                output.write_all(&local_get(0))?;
                output.write_all(&i32_load(2, 0))?;
                output.write_all(&wasm_if(BlockType::Void))?;
                emit_func_body(contents, output)?;
                output.write_all(&br(1))?;
                output.write_all(&end())?;
                output.write_all(&end())?;
            }
        }
    }
    Ok(())
}

fn i32_leb128(value: i32) -> Vec<u8> {
    let mut buf = [0; 5];
    let size = leb128::write::signed(&mut &mut buf[..], value.into()).unwrap();
    Vec::from(&buf[..size])
}

fn u32_leb128(value: u32) -> Vec<u8> {
    let mut buf = [0; 5];
    let size = leb128::write::unsigned(&mut &mut buf[..], value.into()).unwrap();
    Vec::from(&buf[..size])
}
