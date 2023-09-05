use std::cmp;

pub fn compareString(x: &str, y: &str) -> bool {
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
        assert_eq!(compareString("abc", "abc"), false);
        assert_eq!(compareString("abc", "abcd"), false);
        assert_eq!(compareString("abcd", "abc"), true);
        assert_eq!(compareString("abc", "abd"), false);
        assert_eq!(compareString("abd", "abc"), true);
        assert_eq!(compareString("a", "a"), false);
        assert_eq!(compareString("a", "b"), false);
        assert_eq!(compareString("b", "a"), true);
        assert_eq!(compareString("a", ""), true);
        assert_eq!(compareString("", "a"), false);
        assert_eq!(compareString("", ""), false);
    }
}