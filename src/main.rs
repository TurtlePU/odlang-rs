mod ident;
mod typeck;
mod eval;
mod repl;
mod parser;
mod pprint;
mod names;
mod term;

fn main() {
    println!("{:?}", repl::repl());
}
