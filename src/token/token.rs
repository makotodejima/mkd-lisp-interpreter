use crate::expression::expression::Exp;
use anyhow::{bail, Result};

pub fn parse_tokens(tokens: Vec<String>) -> Result<Exp> {
    if tokens.len() == 0 {
        bail!("no token provided")
    }
    let (exp, _) = to_ast(&tokens, 0)?;
    return Ok(exp);
}

// "(+ 10 5)" -> "(", "+", "10", "5", ")"
pub fn tokenize(input: String) -> Vec<String> {
    return input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
}

pub fn atom(token: &str) -> Exp {
    match token {
        "true" => Exp::Boolean(true),
        "false" => Exp::Boolean(false),
        _ => match token.parse::<f64>() {
            Ok(f) => Exp::Number(f),
            Err(_) => Exp::Symbol(token.to_string()),
        },
    }
}

pub fn to_ast(tokens: &[String], start: usize) -> Result<(Exp, usize)> {
    let mut exps: Vec<Exp> = vec![];
    let mut idx = start;

    while idx < tokens.len() {
        match tokens[idx].as_str() {
            "(" => {
                idx += 1;
                let (exp, new_idx) = to_ast(tokens, idx)?;
                idx = new_idx;
                exps.push(exp);
            }
            ")" => {
                if exps.len() < 1 {
                    bail!("Unexpected token )")
                }
                idx += 1;
                break;
            }
            token => {
                idx += 1;
                exps.push(atom(token))
            }
        }
    }

    return Ok((Exp::List(exps), idx));
}
