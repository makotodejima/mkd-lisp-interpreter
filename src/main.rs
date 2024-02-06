mod env;
mod expression;
mod token;

use anyhow::{anyhow, Result};
use env::env::Env;
use expression::expression::Exp;
use token::token::{parse_tokens, tokenize};

// "(+ 10 5)".to_string();
// "( + (- 2 1) (+ 10 5))".to_string();
// "(+ 8 (- 5 4) (+ 4 2))".to_string();
// "( + (- 2 1) (+ 10 5)) (- ( + 1 1) (- 4 4))".to_string();

fn main() -> Result<()> {
    let mut default_env = Env::default();
    loop {
        println!(">>>");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match run(input, &mut default_env) {
            Ok(result) => println!("// {}", result),
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}

fn run(input: String, env: &mut Env) -> Result<Exp> {
    let parsed = parse_tokens(tokenize(input))?;
    // println!("Parsed: {}", out);
    let res = eval(&parsed, env)?;
    Ok(res)
}

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
        Exp::Func(_) => Err(anyhow!("Unexpected function expression")),
    }
}

/* enum MyErr {
    Reason(String),
    Thing(Wat),
    Position(usize, usize),
} */
