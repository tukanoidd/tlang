use binding_usage::BindingUsage;
use block::Block;

use crate::{env::Env, util, val::Val};

mod binding_usage;
mod block;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Number(pub(crate) i32);

impl Number {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, number) = util::extract_digits(s)?;

        Ok((s, Number(number.parse().unwrap())))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn new(s: &str) -> Result<(&str, Self), String> {
        util::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| util::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| util::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| util::tag("/", s).map(|s| (s, Self::Div)))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expr {
    Number(Number),
    Operation { lhs: Number, rhs: Number, op: Op },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {
    #[inline]
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s)
            .or_else(|_| Self::new_number(s))
            .or_else(|_| {
                BindingUsage::new(s)
                    .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
            })
            .or_else(|_| Block::new(s).map(|(s, block)| (s, Self::Block(block))))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Number::new(s)?;
        let (s, _) = util::extract_whitespace(s);

        let (s, op) = Op::new(s)?;
        let (s, _) = util::extract_whitespace(s);

        let (s, rhs) = Number::new(s)?;

        Ok((s, Self::Operation { lhs, rhs, op }))
    }

    #[inline]
    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Expr::Number(Number(n)) => Ok(Val::Number(*n)),
            Expr::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };

                Ok(Val::Number(result))
            }
            Expr::BindingUsage(binding_usage) => binding_usage.eval(env),
            Expr::Block(block) => block.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod number {
        use super::*;

        #[test]
        fn parse_number() {
            assert_eq!(Number::new("132"), Ok(("", Number(132))));
        }
    }

    mod op {
        use super::*;

        #[test]
        fn parse_add_op() {
            assert_eq!(Op::new("+"), Ok(("", Op::Add)));
        }

        #[test]
        fn parse_sub_op() {
            assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
        }

        #[test]
        fn parse_mul_op() {
            assert_eq!(Op::new("*"), Ok(("", Op::Mul)));
        }

        #[test]
        fn parse_div_op() {
            assert_eq!(Op::new("/"), Ok(("", Op::Div)));
        }
    }

    mod expr {
        use super::*;

        use binding_usage::BindingUsage;
        use block::Block;

        use crate::stmt::Stmt;

        mod parse {
            use super::*;

            #[test]
            fn parse_one_plus_two() {
                assert_eq!(
                    Expr::new("1+2"),
                    Ok((
                        "",
                        Expr::Operation {
                            lhs: Number(1),
                            rhs: Number(2),
                            op: Op::Add
                        }
                    ))
                )
            }

            #[test]
            fn parse_expr_with_whitespace() {
                assert_eq!(
                    Expr::new("2 * 2"),
                    Ok((
                        "",
                        Expr::Operation {
                            lhs: Number(2),
                            rhs: Number(2),
                            op: Op::Mul
                        }
                    ))
                );
            }

            #[test]
            fn parse_number_as_expr() {
                assert_eq!(Expr::new("456"), Ok(("", Expr::Number(Number(456)))))
            }

            #[test]
            fn parse_binding_usage() {
                assert_eq!(
                    Expr::new("bar"),
                    Ok((
                        "",
                        Expr::BindingUsage(BindingUsage {
                            name: "bar".to_string(),
                        }),
                    )),
                );
            }

            #[test]
            fn parse_block() {
                assert_eq!(
                    Expr::new("{ 200 }"),
                    Ok((
                        "",
                        Expr::Block(Block {
                            stmts: vec![Stmt::Expr(Expr::Number(Number(200)))],
                        }),
                    )),
                );
            }
        }

        mod eval {
            use super::*;

            use crate::env::Env;

            #[test]
            fn eval_add() {
                assert_eq!(
                    Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(20),
                        op: Op::Add
                    }
                    .eval(&Env::default()),
                    Ok(Val::Number(30))
                );
            }

            #[test]
            fn eval_sub() {
                assert_eq!(
                    Expr::Operation {
                        lhs: Number(1),
                        rhs: Number(5),
                        op: Op::Sub
                    }
                    .eval(&Env::default()),
                    Ok(Val::Number(-4))
                );
            }

            #[test]
            fn eval_mul() {
                assert_eq!(
                    Expr::Operation {
                        lhs: Number(5),
                        rhs: Number(6),
                        op: Op::Mul
                    }
                    .eval(&Env::default()),
                    Ok(Val::Number(30))
                );
            }

            #[test]
            fn eval_div() {
                assert_eq!(
                    Expr::Operation {
                        lhs: Number(200),
                        rhs: Number(20),
                        op: Op::Div
                    }
                    .eval(&Env::default()),
                    Ok(Val::Number(10))
                );
            }

            #[test]
            fn eval_binding_usage() {
                let mut env = Env::default();
                env.store_binding("ten", Val::Number(10));

                assert_eq!(
                    Expr::BindingUsage(BindingUsage {
                        name: "ten".to_string(),
                    })
                    .eval(&env),
                    Ok(Val::Number(10)),
                );
            }
            #[test]
            fn eval_block() {
                assert_eq!(
                    Expr::Block(Block {
                        stmts: vec![Stmt::Expr(Expr::Number(Number(10)))],
                    })
                    .eval(&Env::default()),
                    Ok(Val::Number(10)),
                );
            }
        }
    }
}
