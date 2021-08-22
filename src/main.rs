mod ident;
mod typeck;
mod eval;
mod repl;
mod parser;
mod syntax;
mod multi_result;
mod alpha;
mod names;
mod prelude;
mod input;
mod coordinates;

fn main() {
    println!("{:?}", repl::repl());
}
