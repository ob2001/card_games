use crate::stack::Stack;

#[derive(Debug)]
pub struct Tableau<T: Clone> {
    tableau: Vec<Stack<T>>,
}

impl<T: Clone> Tableau<T> {
    pub fn new(cols: usize) -> Self {
        Tableau { tableau: vec![Stack::new(None); cols] }
    }
}