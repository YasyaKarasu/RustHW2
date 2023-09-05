use std::cmp;

pub fn compare_string(x: &str, y: &str) -> bool {
    let x = x.chars().collect::<Vec<char>>();
    let y = y.chars().collect::<Vec<char>>();
    let xlen = x.len();
    let ylen = y.len();

    if xlen == 0 && ylen == 0 {
        return false;
    }

    if xlen == 0 {
        return false;
    }
    if ylen == 0 {
        return true;
    }

    for i in 0..cmp::min(xlen, ylen) {
        if x[i] < y[i] {
            return false;
        } else if x[i] > y[i] {
            return true;
        }
    }
    return xlen > ylen;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_string() {
        assert_eq!(compare_string("abc", "abc"), false);
        assert_eq!(compare_string("abc", "abcd"), false);
        assert_eq!(compare_string("abcd", "abc"), true);
        assert_eq!(compare_string("abc", "abd"), false);
        assert_eq!(compare_string("abd", "abc"), true);
        assert_eq!(compare_string("a", "a"), false);
        assert_eq!(compare_string("a", "b"), false);
        assert_eq!(compare_string("b", "a"), true);
        assert_eq!(compare_string("a", ""), true);
        assert_eq!(compare_string("", "a"), false);
        assert_eq!(compare_string("", ""), false);
    }
}