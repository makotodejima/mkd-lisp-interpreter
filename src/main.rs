use anyhow::Result;
use mkd_lang::{env::env::Env, run};
use std::{cell::RefCell, rc::Rc};

fn main() -> Result<()> {
    let default_env = Rc::new(RefCell::new(Env::default()));
    loop {
        println!(">>>");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match run(input, default_env.clone()) {
            Ok(result) => println!("// {}", result),
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}
