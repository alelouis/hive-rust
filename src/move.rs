use crate::bug::Bug;
use crate::tile::Direction;
use std::fmt::{Display, Formatter};

pub struct Move {
    source: Bug,
    target: Option<Bug>,
    direction: Direction,
}

impl Move {
    pub fn new(source: Bug, target: Option<Bug>, direction: Direction) -> Move {
        Move {
            source,
            target,
            direction,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let source_str = self.source.to_string();
        let target = self.target;
        match target {
            Some(target_str) => {
                let direction_str = self.direction.to_string();
                let prefix = direction_str
                    .chars()
                    .nth(0)
                    .expect("Couldn't parse prefix.");
                let suffix = direction_str
                    .chars()
                    .nth(1)
                    .expect("Couldn't parse prefix.");
                let move_str = match prefix {
                    '<' => format!("{source_str} {suffix}{target_str}"),
                    '>' => format!("{source_str} {target_str}{suffix}"),
                    _ => panic!("Prefix isn't < or >."),
                };
                write!(f, "{move_str}")
            }
            None => {
                let move_str = format!("{source_str}");
                write!(f, "{move_str}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bug::{Bug, BugKind, Color};
    use crate::r#move::Move;
    use crate::tile::Direction;

    #[test]
    fn move_to_string() {
        let queen = Bug::new(BugKind::Queen, 1, Color::White);
        let ant = Bug::new(BugKind::Ant, 1, Color::White);
        let m = Move::new(ant, Some(queen), Direction::NW);
        assert_eq!(format!("{m}"), "wA1 \\wQ".to_string())
    }
}
