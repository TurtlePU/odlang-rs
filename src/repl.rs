use std::{error::Error, fs::File, io};

use rustyline::{error::ReadlineError, Editor};
use thiserror::Error;

use crate::{
    eval::eval,
    intern::{Context, Term},
    parser::parse,
    pprint::pprint,
    typeck::typeck,
};

const HISTORY_FILE: &'static str = ".odlang_history";

#[derive(Debug, Error)]
pub enum HistoryError {
    #[error("Failed to create history file.")]
    NotCreated(#[from] io::Error),
    #[error("Failed to append history to history file.")]
    NotAppended(#[from] ReadlineError),
}

pub fn repl() -> Result<(), HistoryError> {
    let mut editor = Editor::<()>::new();
    if let Err(_) = editor.load_history(HISTORY_FILE) {
        File::create(HISTORY_FILE)?;
    }
    let mut interner = Context::default();
    while let Ok(line) = editor.readline("turtle > ") {
        if !editor.add_history_entry(&line) {
            println!("This entry will not appear in history.");
        }
        match process_line(&mut interner, &line) {
            Ok(term) => println!("{}", pprint(&interner, term)),
            Err(err) => eprintln!("{}", err),
        }
    }
    Ok(editor.append_history(HISTORY_FILE)?)
}

fn process_line<'a>(
    interner: &mut Context,
    line: &'a str,
) -> Result<Term, Box<dyn Error + 'a>> {
    let term = interner.rename_term(parse(line)?);
    let result = typeck(term.clone());
    if result.errors.is_empty() {
        Ok(eval(term))
    } else {
        Err(result.into())
    }
}
