pub mod env;
mod expression;
mod token;

use anyhow::{anyhow, bail, Result};
use env::env::Env;
use expression::expression::{Exp, Lambda};
use std::{collections::HashMap, rc::Rc};
use token::token::{parse_tokens, tokenize};

pub fn run(input: String, env: &mut Env) -> Result<Exp> {
    let parsed = parse_tokens(tokenize(input))?;
    let res = eval(&parsed, env)?;
    Ok(res)
}

fn eval(expression: &Exp, env: &mut Env) -> Result<Exp> {
    match expression {
        Exp::Symbol(symbol) => {
            return env
                .get_var(symbol)
                .ok_or_else(|| anyhow!("Unexpected symbol: {}", symbol));
        }
        Exp::Number(num) => Ok(Exp::Number(*num)),
        Exp::Boolean(bool) => Ok(Exp::Boolean(*bool)),
        Exp::List(list) => {
            let (first, rest) = list
                .split_first()
                .ok_or_else(|| anyhow!("Failed to parse list items"))?;
            if let Some(keyword_result) = try_builtin_keyword(first, &rest, env)? {
                return Ok(keyword_result);
            } else {
                match eval(first, env)? {
                    Exp::Func(operation) => {
                        let args = rest
                            .iter()
                            .map(|x| eval(x, env))
                            .collect::<Result<Vec<_>>>()?;
                        return operation(&args);
                    }
                    Exp::Lambda(lambda) => {
                        let param_keys = match lambda.params.as_ref() {
                            Exp::List(list) => list
                                .iter()
                                .map(|x| match x {
                                    Exp::Symbol(s) => Ok(s.to_owned()),
                                    _ => bail!("Expected symbol for lambda params"),
                                })
                                .collect::<Result<Vec<_>>>()?,
                            _ => bail!("no"),
                        };

                        if param_keys.len() != rest.len() {
                            bail!("length does not match")
                        }
                        let args = rest
                            .iter()
                            .map(|x| eval(x, env))
                            .collect::<Result<Vec<_>>>()?;

                        let mut data: HashMap<String, Exp> = HashMap::new();

                        for (key, val) in param_keys.iter().zip(args.iter()) {
                            data.insert(key.to_string(), val.clone());
                        }

                        return eval(
                            &lambda.body,
                            &mut Env {
                                data,
                                parent: Some(Box::new(env.clone())),
                            },
                        );
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
        Exp::Lambda(_) => todo!(),
    }
}

fn try_builtin_keyword(expression: &Exp, args: &[Exp], env: &mut Env) -> Result<Option<Exp>> {
    if let Exp::Symbol(keyword) = expression {
        match keyword.as_str() {
            "def" => {
                if args.len() > 2 {
                    bail!("Expected assignment expression")
                }

                let first = args
                    .first()
                    .ok_or_else(|| anyhow!("Failed to find var name"))?;

                if let Exp::Symbol(var_name) = first {
                    let value = args
                        .get(1)
                        .map(|x| eval(x, env))
                        .ok_or_else(|| anyhow!("Could not get value to be assigned"))?
                        .map_err(|e| anyhow!("Evaluation error: {}", e))?;

                    env.data.insert(var_name.to_string(), value.to_owned());
                    return Ok(Some(first.to_owned()));
                }
                bail!("Expected symbol to assign value to")
            }
            "fn" => {
                if args.len() > 2 {
                    bail!("Expected function definition")
                }
                let params = args.first().ok_or_else(|| anyhow!("Expected args"))?;
                let body = args
                    .get(1)
                    .ok_or_else(|| anyhow!("Could not parse function body"))?;

                return Ok(Some(Exp::Lambda(Lambda {
                    params: Rc::new(params.to_owned()),
                    body: Rc::new(body.to_owned()),
                })));
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
