pub struct Buffer<T> {
    data: Vec<T>,
}

impl<T: Default + Copy + std::ops::AddAssign> Buffer<T> {
    pub fn new(data: Vec<T>) -> Self {
        Buffer { data }
    }

    pub fn sum(&self) -> T {
        if self.data.len() == 0 {
            return T::default();
        } else {
            let mut sum = self.data[0];
            for i in 1..self.data.len() {
                sum += self.data[i];
            }
            return sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_sum() {
        let bi32: Buffer<i32> = Buffer::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(bi32.sum(), 15);
        let bi64: Buffer<i64> = Buffer::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(bi64.sum(), 15);
        let bf32: Buffer<f32> = Buffer::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(bf32.sum(), 15.0);
        let bf64: Buffer<f64> = Buffer::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(bf64.sum(), 15.0);
        let bempty: Buffer<i32> = Buffer::new(vec![]);
        assert_eq!(bempty.sum(), 0);
    }
}