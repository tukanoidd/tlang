use crate::{env::Env, expr::Expr, util};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BindingDef {
    pub(crate) name: String,
    pub(crate) val: Expr,
}

impl BindingDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = util::tag("let", s)?;

        let (s, _) = util::extract_whitespace1(s)?;

        let (s, name) = util::extract_ident(s)?;
        let (s, _) = util::extract_whitespace(s);

        let s = util::tag("=", s)?;
        let (s, _) = util::extract_whitespace(s);

        let (s, val) = Expr::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_binding(&self.name, self.val.eval(env)?);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div
                    }
                }
            ))
        );
    }

    #[test]
    fn cannot_parse_binding_def_without_space_after_let() {
        assert_eq!(
            BindingDef::new("letaaa=1+2"),
            Err("expected whitespace".to_string()),
        );
    }
}
