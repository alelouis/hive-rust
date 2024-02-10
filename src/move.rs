use crate::bug::{Bug, BugKind, Color, ParseBugError};
use crate::tile::Direction;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct Move {
    pub source: Bug,
    pub target: Option<Bug>,
    pub direction: Option<Direction>,
}

impl Move {
    pub fn new(source: Bug, target: Option<Bug>, direction: Option<Direction>) -> Move {
        Move {
            source,
            target,
            direction,
        }
    }

    pub fn is_first_piece(&self) -> bool {
        self.target.is_none()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseBugError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut source_str;
        let mut target_str;
        let m = if s.contains(" ") {
            let split: Vec<&str> = s.split(" ").collect();
            (source_str, target_str) = (split.get(0).unwrap(), split.get(1).unwrap());
            let source = Bug::from_str(source_str).expect("Couldn't parse source bug.");
            let dir_char_set = ["/", "\\", "-"];
            let w_move =
                dir_char_set.contains(&target_str.chars().nth(0).unwrap().to_string().as_str());
            let dir_char = if w_move {
                target_str.chars().nth(0).unwrap()
            } else {
                target_str.chars().last().unwrap()
            };

            let direction = match dir_char {
                '/' => {
                    if w_move {
                        Ok(Direction::SW)
                    } else {
                        Ok(Direction::NE)
                    }
                }
                '\\' => {
                    if w_move {
                        Ok(Direction::NW)
                    } else {
                        Ok(Direction::SE)
                    }
                }
                '-' => {
                    if w_move {
                        Ok(Direction::W)
                    } else {
                        Ok(Direction::E)
                    }
                }
                _ => Err(ParseMoveError),
            };
            let target_str_no_dir = target_str.replace(dir_char, "");
            let target = Bug::from_str(target_str_no_dir.as_str())
                .expect("Couldn't parse target bug in move.");
            Move::new(source, Some(target), direction.ok())
        } else {
            let source = Bug::from_str(s).expect("Couldn't parse bug in first move.");
            Move::new(source, None, None)
        };
        Ok(m)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let source_str = self.source.to_string();
        let target = self.target;
        match target {
            Some(target_str) => {
                let direction_str = self
                    .direction
                    .expect("No direction found while target bug was specified.")
                    .to_string();
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
        let m = Move::new(ant, Some(queen), Some(Direction::NW));
        assert_eq!(format!("{m}"), "wA1 \\wQ".to_string())
    }

    #[test]
    fn move_to_string_first_move() {
        let ant = Bug::new(BugKind::Ant, 1, Color::White);
        let m = Move::new(ant, None, Some(Direction::NW));
        assert_eq!(format!("{m}"), "wA1".to_string())
    }
}
