use anyhow::{bail, Result};
use std::{fmt::Display, rc::Rc};

#[derive(Clone)]
pub enum Exp {
    Symbol(String),
    Number(f64),
    Boolean(bool),
    List(Vec<Exp>),
    Func(fn(&[Exp]) -> Result<Exp>),
    Lambda(Lambda),
}

#[derive(Clone)]
pub struct Lambda {
    pub params: Rc<Exp>,
    pub body: Rc<Exp>,
}

impl Exp {
    pub fn as_f64(&self) -> Result<f64> {
        if let Exp::Number(num) = self {
            return Ok(*num);
        }
        bail!("Parse error. Expected a number. Got {}", self.to_string())
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Symbol(v) => write!(f, "{}", v),
            Exp::Number(v) => write!(f, "{}", v),
            Exp::Boolean(v) => write!(f, "Bool({})", v),
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
            Exp::Lambda(lambda) => write!(f, "Lambda({} => {})", lambda.params, lambda.body),
        }
    }
}
