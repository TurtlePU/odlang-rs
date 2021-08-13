use std::{error::Error, fs::File, io};

use rustyline::{error::ReadlineError, Editor};
use thiserror::Error;

use crate::{
    eval::eval,
    ident::identify,
    names::Named,
    parser::parse,
    typeck::{typeck, TypeckResult},
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
    let TypeckResult(_, result) = typeck(alpha, term.clone());
    if result.is_empty() {
        Ok(eval(term).pprint(&names))
    } else {
        Err(result.pprint(&names).into())
    }
}
