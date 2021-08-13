mod ident;
mod typeck;
mod eval;
mod repl;
mod parser;
mod syntax;
mod atoms;

fn main() {
    println!("{:?}", repl::repl());
}
