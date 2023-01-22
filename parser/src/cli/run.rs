use crate::{eval::host::eval, parse::parse};

pub fn run(args: Vec<String>) {
    args[1..].into_iter().for_each(|input| {
        let res = parse(String::from(input));
        match res {
            Ok(ast) => {
                print!("{} ", eval(ast));
            }
            Err(e) => {
                println!(
                    "\n[\x1b[31mError\x1b[0m] {:?}: {}\ninput: \x1b[33m{}\x1b[0m",
                    e.kind(),
                    e,
                    input
                );
                std::process::exit(1);
            }
        }
    });
}