pub trait Indexable<T, S> {
    fn from_index(index: T) -> Option<S>;
    fn to_index(&self) -> T;
}
