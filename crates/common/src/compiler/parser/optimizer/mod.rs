use crate::compiler::parser::syntax::AddUntilZeroArg;
use crate::compiler::parser::syntax::SyntaxTree;

pub trait Rule {
    fn apply(&self, block: SyntaxTree) -> SyntaxTree;
}

pub struct Optimizer {
    rules: Vec<Box<dyn Rule>>,
}

impl Optimizer {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn optimize(&self, mut tree: SyntaxTree) -> SyntaxTree {
        for rule in &self.rules {
            tree = rule.apply(tree);
        }

        match tree {
            SyntaxTree::Root { block } => SyntaxTree::Root {
                block: block.into_iter().map(|tree| self.optimize(tree)).collect(),
            },
            SyntaxTree::Loop { block } => SyntaxTree::Loop {
                block: block.into_iter().map(|tree| self.optimize(tree)).collect(),
            },
            otherwise => otherwise,
        }
    }

    pub fn load_rules(&mut self) {
        self.add_rule(Box::new(ClearRule::new()));
        self.add_rule(Box::new(AddUntilZeroRule::new()));
    }

    fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }
}

pub struct ClearRule;

impl ClearRule {
    pub fn new() -> Self {
        Self
    }
}

impl Rule for ClearRule {
    fn apply(&self, block: SyntaxTree) -> SyntaxTree {
        match block {
            SyntaxTree::Loop { block } => {
                if block.len() == 1 && block[0] == (SyntaxTree::Add { val: -1 }) {
                    SyntaxTree::Clear
                } else {
                    SyntaxTree::Loop { block }
                }
            }
            otherwise => otherwise,
        }
    }
}

pub struct AddUntilZeroRule;

impl AddUntilZeroRule {
    pub fn new() -> Self {
        Self
    }
}

impl Rule for AddUntilZeroRule {
    fn apply(&self, block: SyntaxTree) -> SyntaxTree {
        let block = match block {
            SyntaxTree::Loop { block } => block,
            otherwise => return otherwise,
        };

        // Check whether the first character in code is `-`.
        match block.get(0) {
            Some(SyntaxTree::Add { val: -1 }) => (),
            _ => return SyntaxTree::Loop { block },
        }

        // Check whether the characters after `-` in code start with `<` of `>`.
        match block.get(1) {
            Some(SyntaxTree::Seek { offset: _ }) => (),
            _ => return SyntaxTree::Loop { block },
        }

        let mut current_offset = 0;
        let mut target = Vec::with_capacity(block.len() / 2);

        for statement in block.iter().skip(1) {
            match statement {
                SyntaxTree::Add { val } => {
                    // Optimization fails if the program tries to change the
                    // counter inside a loop.
                    if current_offset == 0 {
                        return SyntaxTree::Loop { block };
                    }

                    target.push(AddUntilZeroArg::new(current_offset, *val))
                }
                SyntaxTree::Seek { offset } => current_offset += *offset as isize,
                _ => return SyntaxTree::Loop { block },
            }
        }

        // Ensure the last behavior is moving the pointer back to the place
        // where it stayed when the loop started.
        if current_offset != 0 {
            SyntaxTree::Loop { block }
        } else {
            SyntaxTree::AddUntilZero { target }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::parser::syntax::AddUntilZeroArg;

    use super::*;

    #[test]
    fn clear_rule() {
        let mut optimizer = Optimizer::new();
        optimizer.add_rule(Box::new(ClearRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![
                SyntaxTree::Input,
                SyntaxTree::Loop {
                    block: vec![SyntaxTree::Add { val: -1 }],
                },
            ],
        };

        let tree = optimizer.optimize(tree);

        let expected = SyntaxTree::Root {
            block: vec![SyntaxTree::Input, SyntaxTree::Clear],
        };

        assert_eq!(tree, expected);
    }

    #[test]
    fn add_until_zero_rule() {
        let mut optimizer = Optimizer::new();
        optimizer.add_rule(Box::new(AddUntilZeroRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![
                SyntaxTree::Loop {
                    block: vec![
                        SyntaxTree::Add { val: -1 },
                        SyntaxTree::Seek { offset: 2 },
                        SyntaxTree::Add { val: -2 },
                        SyntaxTree::Seek { offset: -3 },
                        SyntaxTree::Add { val: 1 },
                        SyntaxTree::Seek { offset: 1 },
                    ],
                },
                SyntaxTree::Loop {
                    block: vec![
                        SyntaxTree::Add { val: -1 },
                        SyntaxTree::Seek { offset: 1 },
                        SyntaxTree::Output,
                        SyntaxTree::Add { val: 1 },
                        SyntaxTree::Seek { offset: -1 },
                    ],
                },
            ],
        };

        let tree = optimizer.optimize(tree);

        let expected = SyntaxTree::Root {
            block: vec![
                SyntaxTree::AddUntilZero {
                    target: vec![AddUntilZeroArg::new(2, -2), AddUntilZeroArg::new(-1, 1)],
                },
                SyntaxTree::Loop {
                    block: vec![
                        SyntaxTree::Add { val: -1 },
                        SyntaxTree::Seek { offset: 1 },
                        SyntaxTree::Output,
                        SyntaxTree::Add { val: 1 },
                        SyntaxTree::Seek { offset: -1 },
                    ],
                },
            ],
        };

        assert_eq!(tree, expected);
    }

    #[test]
    fn add_while_zero_rule_with_changing_the_counter_incorrectly() {
        let mut optimizer = Optimizer::new();
        optimizer.add_rule(Box::new(AddUntilZeroRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![SyntaxTree::Loop {
                block: vec![
                    SyntaxTree::Add { val: -1 },
                    SyntaxTree::Seek { offset: 1 },
                    SyntaxTree::Add { val: 1 },
                    // Move the pointer to the counter and change it apart from
                    // the decrement in the front of the loop.
                    SyntaxTree::Seek { offset: -1 },
                    SyntaxTree::Add { val: -1 },
                ],
            }],
        };

        let tree = optimizer.optimize(tree);

        // Failed to optimize the code and nothing changed
        let expected = SyntaxTree::Root {
            block: vec![SyntaxTree::Loop {
                block: vec![
                    SyntaxTree::Add { val: -1 },
                    SyntaxTree::Seek { offset: 1 },
                    SyntaxTree::Add { val: 1 },
                    SyntaxTree::Seek { offset: -1 },
                    SyntaxTree::Add { val: -1 },
                ],
            }],
        };

        assert_eq!(tree, expected);
    }
}
