#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub from: Position,
    pub to: Position
}
