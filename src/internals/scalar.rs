use super::Scalar;

impl Scalar {
    pub fn dtype(&self) -> String {
        match self {
            &Scalar::i64(_) => "i64".to_string(),
            &Scalar::usize(_) => "usize".to_string(),
            &Scalar::f64(_) => "f64".to_string(),
            &Scalar::bool(_) => "bool".to_string(),
            &Scalar::String(_) => "str".to_string(),
        }
    }

    pub fn is_i64(&self) -> bool {
        match self {
            &Scalar::i64(_) => true,
            _ => false,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            &Scalar::f64(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            &Scalar::bool(_) => true,
            _ => false,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            &Scalar::String(_) => true,
            _ => false,
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            &Scalar::i64(val) => val,
            &Scalar::f64(val) => val as i64,
            &Scalar::bool(val) => val as i64,
            _ => panic!("unable to coerce to i64"),
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            &Scalar::i64(val) => val as f64,
            &Scalar::f64(val) => val,
            _ => panic!("unable to coerce to f64"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            &Scalar::bool(val) => val,
            _ => panic!("unable to coerce to bool"),
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            &Scalar::String(ref val) => val.clone(),
            _ => panic!("unable to coerce to String"),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::super::Scalar;

    #[test]
    fn test_i64_dtype_property() {
        let i = Scalar::i64(1);
        assert_eq!(i.dtype(), "i64".to_string());
        assert_eq!(i.is_i64(), true);
        assert_eq!(i.is_f64(), false);
        assert_eq!(i.is_bool(), false);
        assert_eq!(i.is_str(), false);

        assert_eq!(i.as_i64(), 1);
        assert_eq!(i.as_f64(), 1 as f64);
    }

    #[test]
    #[should_panic]
    fn test_i64_to_bool() {
        // 1 as bool is unsupported cast
        let i = Scalar::i64(1);
        i.as_bool();
    }

    #[test]
    #[should_panic]
    fn test_i64_to_str() {
        // non numeric cast
        let i = Scalar::i64(1);
        i.as_str();
    }

    #[test]
    fn test_f64_dtype_property() {
        let f = Scalar::f64(1.1);
        assert_eq!(f.dtype(), "f64".to_string());
        assert_eq!(f.is_i64(), false);
        assert_eq!(f.is_f64(), true);
        assert_eq!(f.is_bool(), false);
        assert_eq!(f.is_str(), false);

        assert_eq!(f.as_f64(), 1.1);
        assert_eq!(f.as_i64(), 1.1 as i64);
    }

    #[test]
    #[should_panic]
    fn test_f64_to_bool() {
        // 1 as bool is unsupported cast
        let f = Scalar::f64(1.1);
        f.as_bool();
    }

    #[test]
    #[should_panic]
    fn test_f64_to_str() {
        // non numeric cast
        let f = Scalar::f64(1.1);
        f.as_str();
    }

    #[test]
    fn test_bool_dtype_property() {
        let b = Scalar::bool(true);
        assert_eq!(b.dtype(), "bool".to_string());
        assert_eq!(b.is_i64(), false);
        assert_eq!(b.is_f64(), false);
        assert_eq!(b.is_bool(), true);
        assert_eq!(b.is_str(), false);

        assert_eq!(b.as_bool(), true);
        assert_eq!(b.as_i64(), true as i64);
    }

    #[test]
    #[should_panic]
    fn test_bool_to_f64() {
        // casting `bool` as `f64` is invalid
        let b = Scalar::bool(true);
        b.as_f64();
    }

    #[test]
    #[should_panic]
    fn test_bool_to_str() {
        // non numeric cast
        let b = Scalar::bool(true);
        b.as_str();
    }

    #[test]
    fn test_str_dtype_property() {
        let s = Scalar::String("aa".to_string());
        assert_eq!(s.dtype(), "str".to_string());
        assert_eq!(s.is_i64(), false);
        assert_eq!(s.is_f64(), false);
        assert_eq!(s.is_bool(), false);
        assert_eq!(s.is_str(), true);

        assert_eq!(s.as_str(), "aa".to_string());
    }

    #[test]
    #[should_panic]
    fn test_str_to_i64() {
        let s = Scalar::String("1".to_string());
        s.as_i64();
    }

    #[test]
    #[should_panic]
    fn test_str_to_f64() {
        let s = Scalar::String("1.1".to_string());
        s.as_f64();
    }

    #[test]
    #[should_panic]
    fn test_str_to_bool() {
        let s = Scalar::String("true".to_string());
        s.as_bool();
    }
}
