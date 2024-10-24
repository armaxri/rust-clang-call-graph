use serde::Deserialize;
use serde::Serialize;

use super::position::Position;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Range { start, end }
    }

    pub fn create(
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
    ) -> Self {
        Range {
            start: Position::new(start_line, start_column),
            end: Position::new(end_line, end_column),
        }
    }

    pub fn is_position_within_range(&self, position: &Position) -> bool {
        if position.is_position_same_or_after(&self.start)
            && self.end.is_position_same_or_after(position)
        {
            return true;
        }
        false
    }

    pub fn is_within_range_of(&self, other: &Range) -> bool {
        if other.start.is_position_same_or_after(&self.start)
            && self.end.is_position_same_or_after(&other.end)
        {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_position_within_range() {
        let range = Range::new(Position::new(1, 1), Position::new(2, 2));
        let mut other = Position::new(1, 1);
        assert!(range.is_position_within_range(&other));
        other = Position::new(2, 2);
        assert!(range.is_position_within_range(&other));
        other = Position::new(1, 2);
        assert!(range.is_position_within_range(&other));
        other = Position::new(0, 0);
        assert!(!range.is_position_within_range(&other));
        other = Position::new(3, 3);
        assert!(!range.is_position_within_range(&other));
        other = Position::new(2, 3);
        assert!(!range.is_position_within_range(&other));
    }

    #[test]
    fn test_is_within_range_of() {
        let range = Range::new(Position::new(1, 1), Position::new(2, 2));
        let mut other = Range::new(Position::new(1, 1), Position::new(2, 2));
        assert!(range.is_within_range_of(&other));
        other = Range::new(Position::new(1, 1), Position::new(1, 2));
        assert!(range.is_within_range_of(&other));
        other = Range::new(Position::new(2, 1), Position::new(2, 2));
        assert!(range.is_within_range_of(&other));
        other = Range::new(Position::new(1, 1), Position::new(2, 1));
        assert!(range.is_within_range_of(&other));
        other = Range::new(Position::new(0, 0), Position::new(2, 2));
        assert!(!range.is_within_range_of(&other));
        other = Range::new(Position::new(1, 1), Position::new(3, 3));
        assert!(!range.is_within_range_of(&other));
    }
}
