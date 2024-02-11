use crate::expression::expression::Exp;
use anyhow::{anyhow, bail, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

macro_rules! check_sequence_order {
    ($check_fn:expr) => {{
        |args: &[Exp]| -> Result<Exp> {
            let floats = args
                .iter()
                .map(|x| x.as_f64())
                .collect::<Result<Vec<_>>>()?;
            let first = floats.first().ok_or_else(|| anyhow!("err"))?;
            let rest = &floats[1..];
            fn f(prev: &f64, xs: &[f64]) -> bool {
                match xs.first() {
                    Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                    None => true,
                }
            }
            Ok(Exp::Boolean(f(first, rest)))
        }
    }};
}

pub struct Env {
    pub data: HashMap<String, Exp>,
    pub parent_env: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn get_var(&self, symbol: &str) -> Option<Exp> {
        self.data.get(symbol).cloned().or_else(|| {
            self.parent_env
                .as_ref()
                .and_then(|parent_env| parent_env.borrow().get_var(symbol))
        })
    }
}

impl Default for Env {
    fn default() -> Env {
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
                let (first, rest) = floats
                    .split_first()
                    .ok_or_else(|| anyhow!("Could not parse floats for the subtraction"))?;
                return Ok(Exp::Number(first - rest.iter().sum::<f64>()));
            }),
        );

        data.insert(
            "<".to_string(),
            Exp::Func(check_sequence_order!(|a, b| a < b)),
        );
        data.insert(
            ">".to_string(),
            Exp::Func(check_sequence_order!(|a, b| a > b)),
        );
        data.insert(
            "<=".to_string(),
            Exp::Func(check_sequence_order!(|a, b| a <= b)),
        );
        data.insert(
            ">=".to_string(),
            Exp::Func(check_sequence_order!(|a, b| a >= b)),
        );

        data.insert(
            "if".to_string(),
            Exp::Func(|args| {
                let test = args
                    .first()
                    .ok_or_else(|| anyhow!("Condition expected to follow 'if'"))?;
                match test {
                    Exp::Boolean(bool) => {
                        let idx = if *bool { 1 } else { 2 };
                        let exp = args
                            .get(idx)
                            .ok_or_else(|| anyhow!("Expected an expression"))?;
                        return Ok(exp.to_owned());
                    }
                    _ => bail!("Condition expected!"),
                }
            }),
        );

        return Env {
            data,
            parent_env: None,
        };
    }
}
