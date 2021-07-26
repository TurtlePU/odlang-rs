mod bruijn;
mod typeck;
mod eval;
mod repl;
mod parser;

fn main() {
    println!("{:?}", repl::repl());
}
