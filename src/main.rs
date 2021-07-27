mod bruijn;
mod typeck;
mod eval;
mod repl;
mod parser;
mod pprint;

fn main() {
    println!("{:?}", repl::repl());
}
