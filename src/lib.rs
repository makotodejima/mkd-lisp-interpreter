pub mod env;
mod expression;
mod token;

use anyhow::{anyhow, bail, Result};
use env::env::Env;
use expression::expression::Exp;
use token::token::{parse_tokens, tokenize};

pub fn run(input: String, env: &mut Env) -> Result<Exp> {
    let parsed = parse_tokens(tokenize(input))?;
    // println!("Parsed: {}", out);
    let res = eval(&parsed, env)?;
    Ok(res)
}

// struct Lambda {
//     params: Rc<Exp>,
//     body: Rc<Exp>,
// }

fn eval(expression: &Exp, env: &mut Env) -> Result<Exp> {
    match expression {
        Exp::Symbol(symbol) => {
            let operation = env
                .data
                .get(symbol)
                .ok_or_else(|| anyhow!("Unexpected symbol: {}", symbol))?;
            return Ok(operation.to_owned());
        }
        Exp::Number(num) => Ok(Exp::Number(*num)),
        Exp::Boolean(bool) => Ok(Exp::Boolean(*bool)),
        Exp::List(list) => {
            let first = list
                .first()
                .ok_or_else(|| anyhow!("No item found in the list"))?;
            let rest: Vec<Exp> = list.iter().skip(1).cloned().collect();
            if let Some(keyword_result) = handle_keyword(first, &rest, env)? {
                return Ok(keyword_result);
            } else {
                match eval(first, env)? {
                    Exp::Func(operation) => {
                        let args = list
                            .iter()
                            .skip(1)
                            .map(|x| eval(x, env))
                            .collect::<Result<Vec<_>>>()?;
                        return operation(&args);
                    }
                    _ => {
                        let items = list
                            .iter()
                            .map(|x| eval(x, env))
                            .collect::<Result<Vec<_>>>()?;
                        return Ok(Exp::List(items));
                    }
                }
            }
        }
        Exp::Func(_) => Err(anyhow!("Unexpected function expression")),
    }
}

fn handle_keyword(expression: &Exp, args: &[Exp], env: &mut Env) -> Result<Option<Exp>> {
    if let Exp::Symbol(keyword) = expression {
        match keyword.as_str() {
            "def" => {
                let first = args
                    .first()
                    .ok_or_else(|| anyhow!("Failed to find var name"))?;

                if args.len() > 2 {
                    bail!("Expected assignment expression")
                }

                if let Exp::Symbol(var_name) = first {
                    let value = args
                        .get(1)
                        .ok_or_else(|| anyhow!("Could not get value to be assigned"))?;

                    env.data.insert(var_name.to_string(), value.to_owned());
                    return Ok(Some(first.to_owned()));
                }
                bail!("Expected symbol to assign value to")
            }
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

/* enum MyErr {
    Reason(String),
    Thing(Wat),
    Position(usize, usize),
} */
