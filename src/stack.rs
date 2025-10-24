pub struct Stack {
    arr: Vec<u32>,
    cap: usize,
    ptr: usize,
}

#[allow(dead_code)]
impl Stack {
    pub fn new(size: usize) -> Self {
        Stack { arr: Vec::<u32>::with_capacity(size), cap: size, ptr: 0 }
    }
    pub fn add(&mut self, value: u32) {
        self.arr.insert(self.ptr, value);
        self.ptr += 1;
    }
    pub fn pop(&mut self) {
        self.arr[self.ptr] = 0;
        self.ptr -= 1;
    }
    pub fn get(&self) -> u32 {
        self.arr[self.ptr]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_test() {

    }
}