#![allow(unused)]

use crate::parser::syntax::SyntaxTree;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Add(i32),
    Seek(isize),
    Input,
    Output,
    Jump(usize),
    JumpIfZero(usize),
    Halt,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstructionList(pub Vec<Instruction>);

impl InstructionList {
    pub fn compile(syntax_tree: SyntaxTree) -> InstructionList {
        let root = match syntax_tree {
            SyntaxTree::Root(v) => v,
            _ => unreachable!(),
        };

        let mut ins = vec![];
        InstructionList::compile_impl(&mut ins, root);
        ins.push(Instruction::Halt);
        InstructionList(ins)
    }

    fn compile_impl(ins: &mut Vec<Instruction>, syntax_tree: Vec<SyntaxTree>) {
        for node in syntax_tree {
            match node {
                SyntaxTree::Add(val) => ins.push(Instruction::Add(val)),
                SyntaxTree::Seek(offset) => ins.push(Instruction::Seek(offset as isize)),
                SyntaxTree::Input => ins.push(Instruction::Input),
                SyntaxTree::Output => ins.push(Instruction::Output),
                SyntaxTree::Loop(child) => {
                    let loop_start_addr = ins.len();
                    ins.push(Instruction::JumpIfZero(0)); // 0 as a placeholder
                    InstructionList::compile_impl(ins, child);
                    let loop_end_addr = ins.len();
                    ins.push(Instruction::Jump(loop_start_addr));
                    ins[loop_start_addr] = Instruction::JumpIfZero(loop_end_addr + 1);
                }
                SyntaxTree::Root(_) => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::syntax;

    use super::*;

    #[test]
    fn compile() {
        let syntax_tree = SyntaxTree::Root(vec![
            SyntaxTree::Input,
            SyntaxTree::Add(1),
            SyntaxTree::Loop(vec![
                SyntaxTree::Seek(-1),
                SyntaxTree::Add(1),
                SyntaxTree::Seek(1),
                SyntaxTree::Loop(vec![SyntaxTree::Output]),
            ]),
            SyntaxTree::Output,
        ]);

        let ins = InstructionList::compile(syntax_tree);

        let expected = InstructionList(vec![
            Instruction::Input,
            Instruction::Add(1),
            Instruction::JumpIfZero(10),
            Instruction::Seek(-1),
            Instruction::Add(1),
            Instruction::Seek(1),
            Instruction::JumpIfZero(9),
            Instruction::Output,
            Instruction::Jump(6),
            Instruction::Jump(2),
            Instruction::Output,
            Instruction::Halt,
        ]);

        assert_eq!(ins, expected);
    }

    #[test]
    fn compile_from_empty_syntax_tree() {
        let ins = InstructionList::compile(SyntaxTree::Root(vec![]));
        let expected = InstructionList(vec![Instruction::Halt]);
        assert_eq!(ins, expected);
    }
}
