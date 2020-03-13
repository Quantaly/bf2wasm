use super::BrainfuckSyntax;
use super::BrainfuckSyntax::*;

pub fn optimize(ast: &mut Vec<BrainfuckSyntax>) {
    remove_unreachable_loops(ast);
}

pub fn remove_unreachable_loops(ast: &mut Vec<BrainfuckSyntax>) {
    _remove_unreachable_loops(ast, true);
}

fn _remove_unreachable_loops(ast: &mut Vec<BrainfuckSyntax>, first: bool) {
    let mut remove_loop = first;
    for i in 0..ast.len() {
        if let Loop(l) = &mut ast[i] {
            if remove_loop {
                ast[i] = NoOp;
            } else {
                _remove_unreachable_loops(l, false);
                remove_loop = true;
            }
        } else if let NoOp = &ast[i] {
        } else {
            remove_loop = false;
        }
    }
}

#[test]
fn test_remove_unreachable_loops() -> Result<(), super::parser::ParseError> {
    use std::io::Cursor;
    use super::parse_and_optimize;

    let actual = parse_and_optimize(&mut Cursor::new("[this loop is useless, it shouldn't be preserved.]++."))?;
    let expected = vec![NoOp, ModifyValue(2), Output];
    assert_eq!(expected, actual);

    let actual = parse_and_optimize(&mut Cursor::new("+[this is a must-include loop.][this one, however, isn't.]>>"))?;
    let expected = vec![ModifyValue(1), Loop(vec![ModifyValue(-1), Output]), NoOp, MovePointer(2)];
    assert_eq!(expected, actual);

    let actual = parse_and_optimize(&mut Cursor::new("+[[this - the cool loop][this - the uncool loop]]."))?;
    let expected = vec![ModifyValue(1), Loop(vec![Loop(vec![ModifyValue(-1)]), NoOp]), Output];
    assert_eq!(expected, actual);

    Ok(())
}
