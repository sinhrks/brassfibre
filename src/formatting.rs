
use std::string::ToString;

use super::computations;

/// Convert each element in a vector to String
fn to_string_vector<T: ToString>(values: &Vec<T>) -> Vec<String> {
    values.iter().map(|x| x.to_string()).collect()
}

/// Get max number of characters in a vector of String
fn get_width(values: &Vec<String>) -> usize {
    let lens: Vec<usize> = values.iter().map(|x| x.len()).collect();
    computations::vec_max(&lens)
}

fn pad_str(s: &str, pad: usize) -> String {
    let len = s.len();
    // ToDo: fix, can use dynamic formatter in format! macro?
    if len < pad {
        let mut new = "".to_string();
        for _ in 0..(pad - len) {
            new.push(' ');
        }
        new.push_str(s);
        new
    } else {
        s.to_string()
    }
}

/// Convert passed values to Vec of equally padded String
pub fn pad_string_vector<T: ToString>(values: &Vec<T>) -> Vec<String> {
    let strs = to_string_vector(values);
    let pad = get_width(&strs);
    strs.iter().map(|x| pad_str(x, pad)).collect()
}

/// Convert passed values and header to Vec of equally padded String
pub fn pad_string_vector_with_header<T: ToString>(values: &Vec<T>, header: String) -> Vec<String> {
    let mut strs = to_string_vector(values);
    strs.insert(0, header);
    pad_string_vector(&strs)
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_to_string_vector_int() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let s = super::to_string_vector(&values);

        let exp_values: Vec<&str> = vec!["1", "2", "3", "4", "5"];
        assert_eq!(&s, &exp_values);
        assert_eq!(&super::get_width(&s), &1);

        let values: Vec<i64> = vec![10, 200, 30, 4, 50];

        let s = super::to_string_vector(&values);
        let exp_values: Vec<&str> = vec!["10", "200", "30", "4", "50"];
        assert_eq!(&s, &exp_values);
        assert_eq!(&super::get_width(&s), &3);

        let s = super::pad_string_vector(&values);
        let exp_values: Vec<&str> = vec![" 10", "200", " 30", "  4", " 50"];
        assert_eq!(&s, &exp_values);
    }

    #[test]
    fn test_to_string_vector_float() {
        let values: Vec<f64> = vec![1.1, 2.22, 3.5, 4.0, 5.1];
        let s = super::to_string_vector(&values);

        let exp_values: Vec<&str> = vec!["1.1", "2.22", "3.5", "4", "5.1"];
        assert_eq!(&s, &exp_values);
        assert_eq!(&super::get_width(&s), &4);

        let s = super::pad_string_vector(&values);
        let exp_values: Vec<&str> = vec![" 1.1", "2.22", " 3.5", "   4", " 5.1"];
        assert_eq!(&s, &exp_values);
    }

    #[test]
    fn test_to_string_vector_str() {
        let values: Vec<&str> = vec!["AA", "B", "CCCC"];
        let s = super::to_string_vector(&values);

        let exp_values: Vec<&str> = vec!["AA", "B", "CCCC"];
        assert_eq!(&s, &exp_values);
        assert_eq!(&super::get_width(&s), &4);

        let s = super::pad_string_vector(&values);
        let exp_values: Vec<&str> = vec!["  AA", "   B", "CCCC"];
        assert_eq!(&s, &exp_values);
    }

    #[test]
    fn test_to_pad_string_vector_with_header() {
        let values: Vec<&str> = vec!["AA", "B", "CCCC"];
        let s = super::pad_string_vector_with_header(&values, "XXXXX".to_string());

        let exp_values: Vec<&str> = vec!["XXXXX", "   AA", "    B", " CCCC"];
        assert_eq!(&s, &exp_values);
    }
}