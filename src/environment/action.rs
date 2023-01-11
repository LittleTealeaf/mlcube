use std::fmt::Display;

use super::face::Face;

#[repr(u8)]
#[derive(Clone, Copy, Hash)]
pub enum Action {
    Normal(Face),
    Prime(Face),
    Two(Face),
}

impl Action {
    pub fn from_index(index: usize, face: Face) -> Option<Action> {
        match index {
            0 => Some(Action::Normal(face)),
            1 => Some(Action::Prime(face)),
            2 => Some(Action::Two(face)),
            _ => None,
        }
    }

    pub fn from_value(index: usize) -> Option<Action> {
        match Face::from_index(index / 3) {
            Some(face) => Action::from_index(index % 3, face),
            None => None,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            Action::Normal(_) => 0,
            Action::Prime(_) => 1,
            Action::Two(_) => 2,
        }
    }

    pub fn to_value(&self) -> usize {
        match self {
            Action::Normal(face) => 0 + face.to_index() * 3,
            Action::Prime(face) => 1 + face.to_index() * 3,
            Action::Two(face) => 2 + face.to_index() * 3,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Normal(face) => face.to_string(),
                Action::Prime(face) => {
                    let mut string = face.to_string();
                    string.push('\'');
                    string
                }
                Action::Two(face) => {
                    let mut string = face.to_string();
                    string.push('2');
                    string
                }
            }
        )
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Action::Normal(a) => match other {
                Action::Normal(b) => a.eq(b),
                _ => false,
            },
            Action::Prime(a) => match other {
                Action::Prime(b) => a.eq(b),
                _ => false,
            },
            Action::Two(a) => match other {
                Action::Two(b) => a.eq(b),
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_index() {
        let correct_values = [
            (0, Action::Normal(Face::U)),
            (1, Action::Prime(Face::U)),
            (2, Action::Two(Face::U)),
        ];
        for (index, expected) in correct_values {
            let action = Action::from_index(index, Face::U).unwrap();
            assert!(
                action.eq(&expected),
                "from_index should return {} for index {}, instead found {}",
                expected,
                index,
                action
            );
        }

        assert!(
            match Action::from_index(3, Face::U) {
                None => true,
                Some(_) => false,
            },
            "from_index should return None when out-of-bounds index is given"
        );

        assert!(
            match Action::from_index(1, Face::U) {
                Some(_) => true,
                None => false,
            },
            "from_index shoudl return a Some value if a valid index is given"
        );
    }

    #[test]
    fn test_from_value() {
        assert!(
            match Action::from_value(0) {
                Some(_) => true,
                None => false,
            },
            "from_value should return a Some value if a valid index is given"
        );

        assert!(
            match Action::from_value(6 * 3) {
                None => true,
                Some(_) => false,
            },
            "from_value should return a None value when a valid index is given"
        );
    }

    #[test]
    fn test_eq() {
        assert!(
            Action::Normal(Face::R).eq(&Action::Normal(Face::R)),
            "Same Actions should be equal"
        );

        assert!(
            !Action::Two(Face::U).eq(&Action::Prime(Face::U)),
            "Actions with different types should not be equal"
        );

        assert!(
            !Action::Prime(Face::F).eq(&Action::Prime(Face::B)),
            "Actions with the same type but different faces should not be equal"
        );

        assert!(
            !Action::Normal(Face::L).eq(&Action::Two(Face::D)),
            "Actions with different types and faces should not be equal"
        );
    }

    #[test]
    fn test_to_string() {
        let prime = Action::Prime(Face::D);
        assert_eq!(
            prime.to_string(),
            String::from("D'"),
            "to_string should format D prime as D', found {}",
            prime.to_string()
        );

        let two = Action::Two(Face::D);
        assert_eq!(
            two.to_string(),
            String::from("D2"),
            "to_string should format as D2, found {}",
            two.to_string()
        );
    }

    #[test]
    fn to_index_returns_correct_value() {
        for i in 0..=2 {
            let action = Action::from_index(i, Face::U).unwrap();
            assert_eq!(i, action.to_index());
        }
    }

    #[test]
    fn to_value_returns_correct_value() {
        for i in 0..=17 {
            let action = Action::from_value(i).unwrap();
            assert_eq!(i, action.to_value());
        }
    }
}
