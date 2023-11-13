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

//
// impl<T> ArgMax for Vec<T>
// where
//     T: Ord,
// {
//     fn arg_max(&self) -> usize {
//         let (index, _) = self
//             .iter()
//             .enumerate()
//             .max_by_key(|(_, value)| *value)
//             .unwrap();
//         return index;
//     }
// }
//
