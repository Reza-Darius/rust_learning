struct queue_ll<'a> {
    queue: Vec<u32>,
    start: &'a u32,
    end: &'a u32,
    cap: u32,
    len: u32,
}
struct Node<T> {
    value: T,
    next: Box<Node<T>>,
}
struct queue_arr {
    queue: Vec<u32>,
    cap: usize,
    len: u32,
    start: usize,
    end: usize,
}
impl queue_arr {
    fn new(size: usize) -> Self {
        queue_arr{
            queue: Vec::with_capacity(size),
            cap: size,
            len: 0,
            start: 0,
            end: 0,
        }
    }
}


#[cfg(test)]
mod tests {
}