use compiler;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_brainfuck(
    program: &str,
    num_cells: u32,
    cell_size: u32,
    eof: i32,
) -> Result<Vec<u8>, JsValue> {
    if num_cells == 0 {
        return Err(JsValue::from_str("num_cells cannot be 0"));
    }

    let cell_size = match cell_size {
        8 => compiler::CellSize::I8,
        16 => compiler::CellSize::I16,
        32 => compiler::CellSize::I32,
        64 => compiler::CellSize::I64,
        _ => return Err(JsValue::from_str("cell_size must be one of 8, 16, 32, 64")),
    };

    let eof = match eof {
        0 => compiler::EOFBehavior::Zero,
        -1 => compiler::EOFBehavior::NegOne,
        _ => compiler::EOFBehavior::NoChange,
    };

    let ast = match compiler::parse_and_optimize(&mut program.as_bytes()) {
        Ok(ast) => ast,
        Err(e) => return Err(JsValue::from_str(&format!("{}", e))),
    };

    let mut ret = Vec::new();

    match compiler::compile_wasm(
        &ast,
        &compiler::CompilerOptions {
            num_cells,
            cell_size,
            eof
        },
        &mut ret,
    ) {
        Err(e) => return Err(JsValue::from_str(&format!("{}", e))),
        _ => (),
    }

    Ok(ret)
}
