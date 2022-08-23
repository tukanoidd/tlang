use crate::{env::Env, stmt::Stmt, util, val::Val};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Block {
    pub(crate) stmts: Vec<Stmt>,
}

impl Block {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = util::tag("{", s)?;
        let (s, _) = util::extract_whitespace(s);

        let mut s = s;
        let mut stmts = Vec::new();

        while let Ok((new_s, stmt)) = Stmt::new(s) {
            s = new_s;
            stmts.push(stmt);

            let (new_s, _) = util::extract_whitespace(s);
            s = new_s;
        }

        let (s, _) = util::extract_whitespace(s);
        let s = util::tag("}", s)?;

        Ok((s, Block { stmts }))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        if self.stmts.is_empty() {
            Ok(Val::Unit)
        } else {
            let mut child_env = env.create_child();

            let smtms_except_last = &self.stmts[..self.stmts.len() - 1];

            for stmt in smtms_except_last {
                stmt.eval(&mut child_env)?;
            }

            self.stmts.last().unwrap().eval(&mut child_env)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{binding_usage::BindingUsage, Expr, Number},
        *,
    };

    use crate::binding_def::BindingDef;

    mod parse {
        use super::*;

        #[test]
        fn parse_empty_block() {
            assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })));
        }

        #[test]
        fn parse_empty_block_with_whitespace() {
            assert_eq!(Block::new("{   }"), Ok(("", Block { stmts: Vec::new() })));
        }

        #[test]
        fn parse_block_with_one_stmt() {
            assert_eq!(
                Block::new("{ 5 }"),
                Ok((
                    "",
                    Block {
                        stmts: vec![Stmt::Expr(Expr::Number(Number(5)))],
                    },
                )),
            );
        }

        #[test]
        fn parse_block_with_multiple_stmts() {
            assert_eq!(
                Block::new(
                    "{
    let a = 10
    let b = a
    b
}",
                ),
                Ok((
                    "",
                    Block {
                        stmts: vec![
                            Stmt::BindingDef(BindingDef {
                                name: "a".to_string(),
                                val: Expr::Number(Number(10)),
                            }),
                            Stmt::BindingDef(BindingDef {
                                name: "b".to_string(),
                                val: Expr::BindingUsage(BindingUsage {
                                    name: "a".to_string(),
                                }),
                            }),
                            Stmt::Expr(Expr::BindingUsage(BindingUsage {
                                name: "b".to_string(),
                            })),
                        ],
                    },
                )),
            );
        }
    }

    mod eval {
        use super::*;

        use crate::{env::Env, expr::Op, val::Val};

        #[test]
        fn eval_empty_block() {
            assert_eq!(
                Block { stmts: Vec::new() }.eval(&Env::default()),
                Ok(Val::Unit),
            );
        }

        #[test]
        fn eval_block_with_one_expr() {
            assert_eq!(
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(25)))],
                }
                .eval(&Env::default()),
                Ok(Val::Number(25)),
            );
        }

        #[test]
        fn eval_block_with_binding_def_and_usage() {
            assert_eq!(
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "one".to_string(),
                            val: Expr::Number(Number(1)),
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "one".to_string(),
                        })),
                    ],
                }
                .eval(&Env::default()),
                Ok(Val::Number(1)),
            );
        }

        #[test]
        fn eval_block_with_multiple_binding_defs() {
            assert_eq!(
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "foo".to_string(),
                            val: Expr::Number(Number(5)),
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "bar".to_string(),
                            val: Expr::Number(Number(4)),
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "baz".to_string(),
                            val: Expr::Number(Number(3)),
                        }),
                    ],
                }
                .eval(&Env::default()),
                Ok(Val::Unit),
            );
        }

        #[test]
        fn eval_block_with_multiple_exprs() {
            assert_eq!(
                Block {
                    stmts: vec![
                        Stmt::Expr(Expr::Number(Number(100))),
                        Stmt::Expr(Expr::Number(Number(30))),
                        Stmt::Expr(Expr::Operation {
                            lhs: Number(10),
                            rhs: Number(7),
                            op: Op::Sub,
                        }),
                    ],
                }
                .eval(&Env::default()),
                Ok(Val::Number(3)),
            );
        }

        #[test]
        fn eval_block_using_bindings_from_parent_env() {
            let mut env = Env::default();
            env.store_binding("foo", Val::Number(2));

            assert_eq!(
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "baz".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "foo".to_string(),
                            }),
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "baz".to_string(),
                        })),
                    ],
                }
                .eval(&env),
                Ok(Val::Number(2)),
            );
        }
    }
}
