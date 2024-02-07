use anyhow::Result;
use mkd_lang::{env::env::Env, run};

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
