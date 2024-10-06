use super::position::Position;

pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start_line: u32, start_column: u32, end_line: u32, end_column: u32) -> Self {
        Range {
            start: Position::new(start_line, start_column),
            end: Position::new(end_line, end_column),
        }
    }
}
