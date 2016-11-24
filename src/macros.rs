#[macro_export]
macro_rules! array {
    ($($e:expr),*) => ({
        let mut v = Vec::new();
        $(v.push($e);)*
        $crate::Array::new(v)
    });
    ($($e:expr),+,) => (array!($($e),+))
}