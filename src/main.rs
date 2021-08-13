mod ident;
mod typeck;
mod eval;
mod repl;
mod parser;
mod syntax;
mod atoms;
mod multi_result;

fn main() {
    println!("{:?}", repl::repl());
}
