use crate::compiler::parser::{AddUntilZeroArg, SyntaxTree};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Add { val: i32 },
    Seek { offset: isize },
    Clear,
    AddUntilZero { target: Vec<AddUntilZeroArg> },
    Input,
    Output,
    Jump { target: usize },
    JumpIfZero { target: usize },
    Halt,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstructionList(pub Vec<Instruction>);

impl InstructionList {
    pub fn compile(syntax_tree: SyntaxTree) -> InstructionList {
        let root = match syntax_tree {
            SyntaxTree::Root { block: v } => v,
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
                SyntaxTree::Add { val } => ins.push(Instruction::Add { val }),
                SyntaxTree::Seek { offset } => ins.push(Instruction::Seek {
                    offset: offset as isize,
                }),
                SyntaxTree::Clear => ins.push(Instruction::Clear),
                SyntaxTree::AddUntilZero { target } => {
                    ins.push(Instruction::AddUntilZero { target })
                }
                SyntaxTree::Input => ins.push(Instruction::Input),
                SyntaxTree::Output => ins.push(Instruction::Output),
                SyntaxTree::Loop { block } => {
                    let loop_start_addr = ins.len();
                    ins.push(Instruction::JumpIfZero { target: 0 }); // 0 as a placeholder
                    InstructionList::compile_impl(ins, block);
                    let loop_end_addr = ins.len();
                    ins.push(Instruction::Jump {
                        target: loop_start_addr,
                    });
                    ins[loop_start_addr] = Instruction::JumpIfZero {
                        target: loop_end_addr + 1,
                    };
                }
                SyntaxTree::Root { block: _ } => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile() {
        let syntax_tree = SyntaxTree::Root {
            block: vec![
                SyntaxTree::Input,
                SyntaxTree::Add { val: 1 },
                SyntaxTree::Loop {
                    block: vec![
                        SyntaxTree::Seek { offset: -1 },
                        SyntaxTree::Add { val: 1 },
                        SyntaxTree::Seek { offset: 1 },
                        SyntaxTree::Loop {
                            block: vec![SyntaxTree::Output],
                        },
                    ],
                },
                SyntaxTree::Output,
            ],
        };

        let ins = InstructionList::compile(syntax_tree);

        let expected = InstructionList(vec![
            Instruction::Input,
            Instruction::Add { val: 1 },
            Instruction::JumpIfZero { target: 10 },
            Instruction::Seek { offset: -1 },
            Instruction::Add { val: 1 },
            Instruction::Seek { offset: 1 },
            Instruction::JumpIfZero { target: 9 },
            Instruction::Output,
            Instruction::Jump { target: 6 },
            Instruction::Jump { target: 2 },
            Instruction::Output,
            Instruction::Halt,
        ]);

        assert_eq!(ins, expected);
    }

    #[test]
    fn compile_from_empty_syntax_tree() {
        let ins = InstructionList::compile(SyntaxTree::Root { block: vec![] });
        let expected = InstructionList(vec![Instruction::Halt]);
        assert_eq!(ins, expected);
    }
}
