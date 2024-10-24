use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    pub fn is_before(&self, other: &Position) -> bool {
        if self.line < other.line {
            return true;
        } else if self.line == other.line {
            return self.column < other.column;
        }
        false
    }

    pub fn is_position_same_or_after(&self, other: &Position) -> bool {
        return !self.is_before(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_before() {
        let position = Position::new(1, 1);
        let mut other = Position::new(2, 2);
        assert!(position.is_before(&other));
        other = Position::new(1, 1);
        assert!(!position.is_before(&other));
        other = Position::new(0, 0);
        assert!(!position.is_before(&other));
    }

    #[test]
    fn test_is_position_same_or_after() {
        let position = Position::new(1, 1);
        let mut other = Position::new(0, 0);
        assert!(position.is_position_same_or_after(&other));
        other = Position::new(1, 0);
        assert!(position.is_position_same_or_after(&other));
        other = Position::new(1, 1);
        assert!(position.is_position_same_or_after(&other));
        other = Position::new(2, 2);
        assert!(!position.is_position_same_or_after(&other));
    }
}
