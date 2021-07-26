mod bruijn;
mod typeck;
mod eval;
mod repl;
mod parser;
mod var;

fn main() {
    println!("{:?}", repl::repl());
}
