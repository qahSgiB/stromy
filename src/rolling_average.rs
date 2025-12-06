use std::ops::{AddAssign, Div, SubAssign};



pub struct RollingAvegare<T, const BUFFER_SIZE: usize> {
    buffer: [T; BUFFER_SIZE],
    sum: T,
    start: usize,
    count: usize,
}

// TODO: optimize for numeric, Default to zero, MaybeUninit buffer
impl<T: Default + Copy, const BUFFER_SIZE: usize> RollingAvegare<T, BUFFER_SIZE> {
    pub fn new() -> Self {
        let zero = T::default();

        Self {
            buffer: [zero; BUFFER_SIZE],
            sum: zero,
            start: 0,
            count: 0,
        }
    }
}

impl<T: SubAssign + AddAssign + Copy, const BUFFER_SIZE: usize> RollingAvegare<T, BUFFER_SIZE> {
    pub fn add(&mut self, value: T) {
        // SAFETY: we know that start is valid index
        let start_item = unsafe { self.buffer.get_unchecked_mut(self.start) };
        
        self.sum -= *start_item;
        self.sum += value;

        *start_item = value;

        if self.start == BUFFER_SIZE - 1 {
            self.start = 0;
        } else {
            self.start += 1;
        }

        if self.count != BUFFER_SIZE {
            self.count += 1;
        }
    }
}

impl<T: Copy, const BUFFER_SIZE: usize> RollingAvegare<T, BUFFER_SIZE> {
    pub fn get_average<Output>(&self) -> Output
    where
        Output: Div<Output = Output>,
        Output: From<T>,
        Output: From<usize>,
    {
        // hope from is optimized for numeric types
        Output::from(self.sum) / Output::from(self.count)
    }
}

impl<T: Copy + Div<Output = T> + From<usize>, const BUFFER_SIZE: usize> RollingAvegare<T, BUFFER_SIZE> {
    pub fn get_average_t(&self) -> T {
        self.get_average()
    }
}

// TODO
impl<const BUFFER_SIZE: usize> RollingAvegare<u32, BUFFER_SIZE> {
    pub fn get_average_f32(&self) -> f32 {
        (self.sum as f32) / (self.count as f32)
    }
}