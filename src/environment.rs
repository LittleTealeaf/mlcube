pub struct Environment {
    state: Vec<u8>
}

impl Environment {
    pub fn new() -> Environment {
        let mut state: Vec<u8> = Vec::new();
        for c in 0..6 {
            for _ in 0..9 {
                state.push(c);
            }
        }

        Environment { state }
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


    pub fn scramble(&mut self) {
    }
}


mod tests {
    use super::*;


    #[test]
    fn new_cube_is_solved() {
        let env = Environment::new();
        assert!(env.is_solved());
    }

}
