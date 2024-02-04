use anyhow::{anyhow, bail, Result};
use std::{collections::HashMap, fmt::Display};

fn main() -> Result<()> {
    let exp = "(+ 8 (- 5 4) (+ 4 2)".to_string();
    let exp_2 = "( + (- 2 1) (+ 10 5)".to_string();
    let exp_3 = "( + (- 2 1) (+ 10 5)) (- ( + 1 1) (- 4 4)  )".to_string();
    let tokens = tokenize(exp);
    let out = parse_tokens(tokens)?;
    // println!("{:?}", tokens);
    // println!("Hello, world!");
    Ok(())
}

// 1. tokenization
// 2. build AST (Abstract Syntax Tree)

enum Exp {
    Symbol(String),
    Number(f64),
    List(Vec<Exp>),
    Func(fn(&[Exp]) -> Result<Exp>),
}

struct Env {
    data: HashMap<String, Exp>,
}

impl Default for Env {
    fn default() -> Self {
        let mut data: HashMap<String, Exp> = HashMap::new();

        data.insert(
            "+".to_string(),
            Exp::Func(|args| {
                let floats = args
                    .iter()
                    .map(|x| x.as_f64())
                    .collect::<Result<Vec<_>>>()?;
                return Ok(Exp::Number(floats.iter().sum()));
            }),
        );

        data.insert(
            "-".to_string(),
            Exp::Func(|args| {
                let floats = args
                    .iter()
                    .map(|x| x.as_f64())
                    .collect::<Result<Vec<_>>>()?;
                let first = floats.first().ok_or_else(|| {
                    anyhow!("Could not get the first element for the subtraction")
                })?;
                return Ok(Exp::Number(first - floats.iter().skip(1).sum::<f64>()));
            }),
        );

        return Self { data };
    }
}


fn eval(exp: &Exp, mut env:  Env) {
    match exp {
        Exp::Symbol(symbol) => {

        },
        Exp::Number(_) => todo!(),
        Exp::List(_) => todo!(),
        Exp::Func(_) => todo!(),
    }

}

impl Exp {
    fn as_f64(&self) -> Result<f64> {
        if let Exp::Number(num) = self {
            return Ok(*num);
        }
        bail!("Parse error. Expected a number. Got {}", self.to_string())
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Symbol(v) => write!(f, "S({})", v),
            Exp::Number(v) => write!(f, "N({})", v),
            Exp::List(values) => {
                write!(f, "[")?;
                for (i, v) in values.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Exp::Func(_) => write!(f, "Func(<fn>)"),
        }
    }
}

enum MyErr {
    Reason(String),
}

// tokenize("(+ 10 5)") //=> ["(", "+", "10", "5", ")"]
// tokenize("( +( + (- 2 1) (+ 10 5) ( - ( + 1 1) (- 4 4) ) )") //=> ["(", "+", "(", "-", "2", "1", ")", "(", "+" "10", "5", ")", ")"]

fn tokenize(input: String) -> Vec<String> {
    return input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
}

fn atom(token: &str) -> Exp {
    match token.parse::<f64>() {
        Ok(f) => Exp::Number(f),
        Err(_) => Exp::Symbol(token.to_string()),
    }
}

fn to_ast(tokens: &[String], start: usize) -> Result<(Exp, usize)> {
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

fn parse_tokens(tokens: Vec<String>) -> Result<Exp> {
    if tokens.len() == 0 {
        bail!("no token provided")
    }
    let (exp, _) = to_ast(&tokens, 0)?;
    return Ok(exp);
}
