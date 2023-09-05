pub fn self_increment(v: &Vec<char>) -> Vec<char> {
    v.iter()
        .map(|x| char::from_u32((*x as u32) + 1)
        .unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_increment() {
        let mut s = vec!['a', 'b', 'c', 'd', 'e'];
        s = self_increment(&s);
        assert_eq!(s, vec!['b', 'c', 'd', 'e', 'f']);
    }
}