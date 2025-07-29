/// Rolling Buffer that will fill to capacity and then start overwriting the oldest data first.
///
/// Example
/// ```ignore
/// let mut buff = RollingBuffer::new(20);
/// for num in 0..40 {
///     buff.add(num);
///     let vec_out = buff.values();
///     let iter_out: Vec<_> = buff.values_iter().cloned().collect();
///     println!("len: {} - {:?}", buff.len(), vec_out);
///     println!("len: {} - {:?}", buff.len(), iter_out);
///     assert_eq!(vec_out, iter_out);
/// }
/// ```

#[derive(Default)]
pub struct RollingBuffer<T>
where
    T: Clone + Default,
{
    capacity: usize,
    buffer: Vec<T>,
    current_index: usize,
    count: usize,
}
#[allow(dead_code)]
impl<T> RollingBuffer<T>
where
    T: Clone + Default,
{
    /// Create a new buffer and allocate memory for it immediately up to a size of capacity.
    pub fn new(capacity: usize) -> RollingBuffer<T> {
        RollingBuffer {
            capacity,
            buffer: vec![Default::default(); capacity],
            current_index: 0,
            count: 0,
        }
    }
    /// Add a new value to the buffer, if it has reached capacity it will overwrite the oldest datapoint.
    pub fn add(&mut self, value: T) -> bool {
        self.buffer[self.current_index] = value;
        self.current_index = (self.current_index + 1) % self.capacity;
        self.count = std::cmp::min(self.count + 1, self.capacity);
        self.is_full()
    }
    /// How many valid values are in the buffer?
    pub fn len(&self) -> usize {
        self.count
    }
    /// Report is the buffer has reached capacity and is now overwriting old data.
    pub fn is_full(&self) -> bool {
        self.count == self.capacity
    }
    /// Return a single value from the buffer at the position requested.  
    /// If the buffer has hit capacity this will be the position in the valid data -
    /// where index 0 is the newest datapoint, 1 is the second newest, and so on.
    ///
    /// If index > capacity, returns the oldest valid number
    pub fn get(&self, index: usize) -> &T {
        if index >= self.capacity {
            return &self.buffer[(self.current_index + self.capacity - 1 + self.capacity
                - self.count)
                % self.capacity];
        }
        &self.buffer[(self.current_index + index + self.capacity - self.count) % self.capacity]
    }
    /// Return all of the valid values in the buffer, in the order they were added as a Vector.
    pub fn values(&self) -> Vec<T> {
        let mut values = Vec::with_capacity(self.count);
        for i in 0..self.count {
            values.push(self.get(i).clone());
        }
        values
    }
    /// Return all of the vald values in the buffer, in the order they were added, as an Iterator.
    ///
    /// Simple example:
    /// ```ignore
    /// let value_as_vec: Vec<_> = buff.values_iter().cloned().collect();
    /// ```
    ///
    pub fn values_iter(&self) -> impl Iterator<Item = &T> {
        (0..self.count).map(move |i| self.get(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    #[ignore = "debug only"]
    #[test]
    fn test_timing() {
        let start = Instant::now();
        let mut buff = RollingBuffer::new(20);
        for num in 0..40 {
            let val = num;
            buff.add(val);
            println!("len: {} - {:?}", buff.len(), buff.values());
            let output: Vec<&i32> = buff.values_iter().collect();
            println!("len: {} - {:?}", buff.len(), output);
        }
        println!("runtime: {:?}", start.elapsed());
        assert_eq!(5, 0); // designed to fail
    }
    #[test]
    fn test_values_and_values_iter_equal() {
        let mut buff = RollingBuffer::new(20);
        for num in 0..40 {
            // let num = ALPHABET.chars().nth(num).unwrap_or('Z');
            buff.add(num);
            let vec_out = buff.values();
            let iter_out: Vec<_> = buff.values_iter().cloned().collect();
            println!("len: {} - {:?}", buff.len(), vec_out);
            println!("len: {} - {:?}", buff.len(), iter_out);
            assert_eq!(vec_out, iter_out);
        }
    }
    #[ignore = "debug only"]
    #[test]
    fn test_enumerate() {
        let mut buff = RollingBuffer::new(20);
        for num in 0..40 {
            buff.add(num);
        }
        let data: Vec<(usize, _)> = buff.values_iter().cloned().enumerate().collect();
        println!("{data:?}");
        assert_eq!(5, 0) // designed to fail
    }
    #[test]
    fn test_get_overflow() {
        let mut buff = RollingBuffer::new(20);
        for num in 0..40 {
            buff.add(num);
        }
        assert_eq!(*buff.get(1000), 39);
    }
    #[test]
    fn test_char_vec() {
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut buff = RollingBuffer::new(20);
        for num in 0..40 {
            let num = alphabet.chars().nth(num).unwrap_or('Z');
            buff.add(num);
            let vec_out = buff.values();
            let iter_out: Vec<_> = buff.values_iter().cloned().collect();
            assert_eq!(vec_out, iter_out);
        }
    }
    #[test]
    fn test_struct() {
        let mut buff = RollingBuffer::new(20);
        #[derive(Default, Clone, Copy, PartialEq, Debug)]
        struct Data {
            x: f32,
            y: f32,
        }
        for num in 0..40_u8 {
            let num: f32 = num.into();
            let point = Data {
                x: num,
                y: num * 5.3,
            };
            buff.add(point);
            let vec_out = buff.values();
            let iter_out: Vec<_> = buff.values_iter().cloned().collect();
            assert_eq!(vec_out, iter_out);
        }
    }
}
