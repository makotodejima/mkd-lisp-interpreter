use crate::expression::expression::Exp;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

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

        return Self { data };
    }
}
