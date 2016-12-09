
pub struct Elemwise;

/// perform broadcast and elemwise op avoiding unnecessary clone
impl Elemwise {

    pub fn broadcast_oo<T, V, R, F>(values: Vec<T>, _rhs: V, func: F) -> Vec<R>
        where V: Clone,
              F: Fn(T, V) -> R {

        let new_values: Vec<R> = values.into_iter()
                                       .map(|x| func(x, _rhs.clone()))
                                       .collect();
        new_values
    }

    pub fn broadcast_or<T, V, R, F>(values: Vec<T>, _rhs: &V, func: F) -> Vec<R>
        where V: Clone,
              F: Fn(T, V) -> R {

        let new_values: Vec<R> = values.into_iter()
                                       .map(|x| func(x, _rhs.clone()))
                                       .collect();
        new_values
    }

    pub fn broadcast_ro<T, V, R, F>(values: &Vec<T>, _rhs: V, func: F) -> Vec<R>
        where T: Clone,
              V: Clone,
              F: Fn(T, V) -> R {

        let new_values: Vec<R> = values.iter()
                                       .map(|x| func(x.clone(), _rhs.clone()))
                                       .collect();
        new_values
    }

    pub fn broadcast_rr<T, V, R, F>(values: &Vec<T>, _rhs: &V, func: F) -> Vec<R>
        where T: Clone,
              V: Clone,
              F: Fn(T, V) -> R {

        let new_values: Vec<R> = values.iter()
                                       .map(|x| func(x.clone(), _rhs.clone()))
                                       .collect();
        new_values
    }

    pub fn elemwise_oo<T, V, R, F>(values: Vec<T>, _rhs: Vec<V>, func: F) -> Vec<R>
        where F: Fn(T, V) -> R {

        assert!(values.len() == _rhs.len(), "lhs and rhs must be the same length");
        let new_values: Vec<R> = values.into_iter()
                                       .zip(_rhs.into_iter())
                                       .map(|(x, y)| func(x, y))
                                       .collect();
        new_values
    }

    pub fn elemwise_or<T, V, R, F>(values: Vec<T>, _rhs: &Vec<V>, func: F) -> Vec<R>
        where V: Clone,
              F: Fn(T, V) -> R {

        assert!(values.len() == _rhs.len(), "lhs and rhs must be the same length");
        let new_values: Vec<R> = values.into_iter()
                                       .zip(_rhs.iter())
                                       .map(|(x, y)| func(x, y.clone()))
                                       .collect();
        new_values
    }

    pub fn elemwise_ro<T, V, R, F>(values: &Vec<T>, _rhs: Vec<V>, func: F) -> Vec<R>
        where T: Clone,
              F: Fn(T, V) -> R {

        assert!(values.len() == _rhs.len(), "lhs and rhs must be the same length");
        let new_values: Vec<R> = values.iter()
                                       .zip(_rhs.into_iter())
                                       .map(|(x, y)| func(x.clone(), y))
                                       .collect();
        new_values
    }

    pub fn elemwise_rr<T, V, R, F>(values: &Vec<T>, _rhs: &Vec<V>, func: F) -> Vec<R>
        where T: Clone,
              V: Clone,
              F: Fn(T, V) -> R {

        assert!(values.len() == _rhs.len(), "lhs and rhs must be the same length");
        let new_values: Vec<R> = values.iter()
                                       .zip(_rhs.iter())
                                       .map(|(x, y)| func(x.clone(), y.clone()))
                                       .collect();
        new_values
    }
}

#[cfg(test)]
mod tests {

    use super::Elemwise;

    #[test]
    fn test_broadcast() {
        let res = Elemwise::broadcast_oo(vec![1, 2, 3], 2, |x, y| x + y);
        assert_eq!(res, vec![3, 4, 5]);

        let res = Elemwise::broadcast_or(vec![1, 2, 3], &2, |x, y| x + y);
        assert_eq!(res, vec![3, 4, 5]);

        let res = Elemwise::broadcast_ro(&vec![1, 2, 3], 2, |x, y| x + y);
        assert_eq!(res, vec![3, 4, 5]);

        let res = Elemwise::broadcast_rr(&vec![1, 2, 3], &2, |x, y| x + y);
        assert_eq!(res, vec![3, 4, 5]);
    }

    #[test]
    fn test_elemwise() {
        let res = Elemwise::elemwise_oo(vec![1, 2, 3], vec![2, 2, 2], |x, y| x + y);
        assert_eq!(res, vec![3, 4, 5]);

        let res = Elemwise::elemwise_or(vec![1, 2, 3], &vec![2, 2, 2], |x, y| x + y);
        assert_eq!(res, vec![3, 4, 5]);

        let res = Elemwise::elemwise_ro(&vec![1, 2, 3], vec![2, 2, 2], |x, y| x + y);
        assert_eq!(res, vec![3, 4, 5]);

        let res = Elemwise::elemwise_rr(&vec![1, 2, 3], &vec![2, 2, 2], |x, y| x + y);
        assert_eq!(res, vec![3, 4, 5]);
    }

    #[test]
    #[should_panic]
    fn test_elemwise_oo_length_mismatch() {
        Elemwise::elemwise_oo(vec![1, 2, 3], vec![2, 2], |x, y| x + y);
    }

    #[test]
    #[should_panic]
    fn test_elemwise_ro_length_mismatch() {
        Elemwise::elemwise_ro(&vec![1, 2, 3], vec![2, 2], |x, y| x + y);
    }

    #[test]
    #[should_panic]
    fn test_elemwise_or_length_mismatch() {
        Elemwise::elemwise_or(vec![1, 2, 3], &vec![2, 2], |x, y| x + y);
    }

    #[test]
    #[should_panic]
    fn test_elemwise_rr_length_mismatch() {
        Elemwise::elemwise_rr(&vec![1, 2, 3], &vec![2, 2], |x, y| x + y);
    }
}