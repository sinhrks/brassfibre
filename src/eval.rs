
pub trait Applicable<T, R, C> {

    // T: Type for myself
    // R: Type function returns
    // C: Type of container which can hold W as values

    fn apply(&self, func: &Fn(&Vec<T>) -> R) -> C;
}
