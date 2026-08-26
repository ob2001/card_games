#[derive(Clone, Debug)]
pub struct Stack<T: Clone> {
    stack: Vec<T>,
    lim: Option<usize>
}

impl<T: Clone> Stack<T> {
    pub fn new(lim: Option<usize>) -> Self {
        Stack { stack: vec![], lim }
    }
}