use cli::{run::run};
mod cli;
mod eval;
mod parse;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 0 {
        println!("argument is not provided.");
        std::process::exit(1);
    }
    run(args);
}