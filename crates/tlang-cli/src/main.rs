use std::{io, io::Write};

fn run(input: &str, env: &mut tlang::Env) -> Result<Option<tlang::Val>, String> {
    let parse = tlang::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;

    let evaluated = parse
        .eval(env)
        .map_err(|msg| format!("Evaluation error: {}", msg))?;

    if evaluated == tlang::Val::Unit {
        Ok(None)
    } else {
        Ok(Some(evaluated))
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();
    let mut env = tlang::Env::default();

    loop {
        write!(stdout, "→ ")?;
        io::stdout().flush()?;

        stdin.read_line(&mut input)?;

        match run(input.trim(), &mut env) {
            Ok(Some(val)) => writeln!(stdout, "{}", val)?,
            Err(msg) => writeln!(stderr, "{}", msg)?,
            _ => {}
        }

        input.clear();
    }
}
