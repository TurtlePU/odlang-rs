mod ident;
mod typeck;
mod eval;
mod repl;
mod parser;
mod pprint;
mod names;

fn main() {
    println!("{:?}", repl::repl());
}
