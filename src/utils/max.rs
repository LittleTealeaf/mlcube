pub trait ArgMax {
    fn arg_max(&self) -> usize;
}

impl ArgMax for Vec<f64> {
    fn arg_max(&self) -> usize {
        let mut max_index = 0;
        for index in 0..self.len() {
            if self[index] > self[max_index] {
                max_index = index;
            }
        }
        return max_index;
    }
}

pub trait Max<T> {
    fn max(&self) -> T;
}

impl Max<f64> for Vec<f64> {
    fn max(&self) -> f64 {
        let mut max = self[0];
        for value in &self[1..] {
            max = max.max(*value);
        }

        max
    }
}
