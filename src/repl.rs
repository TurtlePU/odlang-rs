use std::{error::Error, fs::File, io};

use rustyline::{error::ReadlineError, Editor};
use thiserror::Error;

use crate::{
    eval::eval,
    ident::identify,
    atoms::Named,
    parser::parse,
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
    while let Ok(line) = editor.readline("turtle > ") {
        if !editor.add_history_entry(&line) {
            println!("This entry will not appear in history.");
        }
        match process_line(&line) {
            Ok(line) => println!("{}", line),
            Err(err) => eprintln!("{}", err),
        }
    }
    Ok(editor.append_history(HISTORY_FILE)?)
}

fn process_line<'a>(
    line: &'a str,
) -> Result<String, Box<dyn Error + 'a>> {
    let (term, names, alpha) = identify(parse(line)?)?;
    match typeck(alpha, term.clone()) {
        Ok(_) => Ok(eval(term).pprint(&names)),
        Err(err) => Err(err.pprint(&names).into()),
    }
}
