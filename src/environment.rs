pub struct Action {
    name: String,
    perms: Vec<Vec<usize>>,
}

impl Action {
    fn new(name: String, perms: Vec<Vec<usize>>) -> Action {
        Action { name, perms }
    }

    fn create_set(name: String, perms: Vec<Vec<usize>>) -> (Action, Action, Action) {
        let mut name_two = name.clone();
        name_two.push('2');
        let mut perms_two: Vec<Vec<usize>> = Vec::new();
        for row in &perms {
            let mut row_two_a: Vec<usize> = Vec::new();
            let mut row_two_b: Vec<usize> = Vec::new();
            for i in 0..row.len() {
                if i % 2 == 0 {
                    row_two_a.push(row[i]);
                } else {
                    row_two_b.push(row[i]);
                }
            }
            perms_two.push(row_two_a);
            perms_two.push(row_two_b);
        }
        let action_two = Action::new(name_two, perms_two);

        let mut name_prime = name.clone();
        name_prime.push('\'');
        let mut perms_prime: Vec<Vec<usize>> = Vec::new();
        for row in &perms {
            let mut row_prime: Vec<usize> = Vec::new();
            for i in row {
                row_prime.push(*i);
            }
            row_prime.reverse();
            perms_prime.push(row_prime);
        }
        let action_prime = Action::new(name_prime, perms_prime);

        let action = Action::new(name, perms);

        (action, action_prime, action_two)
    }
}

pub struct Cube {
    state: Vec<u8>,
}

impl Cube {
    pub fn new() -> Cube {
        let mut state: Vec<u8> = Vec::new();
        for c in 0..6 {
            for _ in 0..9 {
                state.push(c);
            }
        }

        Cube { state }
    }

    pub fn is_solved(&self) -> bool {
        let state = &self.state;
        for i in 0..state.len() {
            if state[i] != (i / 9) as u8 {
                return false;
            }
        }
        true
    }

    pub fn apply(&mut self, action: Action) {
        for perm in action.perms {
            let first = self.state[perm[0]];
            for i in 0..(perm.len() - 1) {
                self.state[perm[i]] = self.state[perm[i + 1]];
            }
            self.state[perm[perm.len() - 1]] = first;
        }
    }

    pub fn scramble(&mut self) {}
}

mod tests {
    use super::*;

    mod action {
        use super::Action;

        #[test]
        fn new_action_sets_name() {
            let action = Action::new(
                String::from("R"),
                vec![vec![20, 2, 42, 47], vec![23, 5, 39, 50]],
            );
            assert_eq!(action.name, String::from("R"));
        }

        #[test]
        fn new_action_sets_perms() {
            let action = Action::new(
                String::from("R"),
                vec![vec![20, 2, 42, 47], vec![23, 5, 39, 50]],
            );
            assert_eq!(action.perms, vec![vec![20, 2, 42, 47], vec![23, 5, 39, 50]]);
        }

        #[test]
        fn create_action_set_configures_names() {
            let (base, prime, two) = Action::create_set(
                String::from("R"),
                vec![vec![20, 2, 42, 47], vec![23, 5, 39, 50]],
            );
            assert_eq!(base.name, String::from("R"));
            assert_eq!(prime.name, String::from("R'"));
            assert_eq!(two.name, String::from("R2"));
        }

        #[test]
        fn create_action_set_configures_perms() {
            let (base, prime, two) = Action::create_set(
                String::from("R"),
                vec![vec![20, 2, 42, 47], vec![23, 5, 39, 50]],
            );
            assert_eq!(base.perms, vec![vec![20, 2, 42, 47], vec![23, 5, 39, 50]]);
            assert_eq!(prime.perms, vec![vec![47, 42, 2, 20], vec![50, 39, 5, 23]]);
            assert_eq!(
                two.perms,
                vec![vec![20, 42], vec![2, 47], vec![23, 39], vec![5, 50]]
            );
        }
    }

    mod cube {

        use super::super::Cube;

        #[test]
        fn new_cube_is_solved() {
            let env = Cube::new();
            assert!(env.is_solved());
        }
    }
}
