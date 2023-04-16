
pub struct Bag<T>(scc::Bag<T>);

impl<T> Bag<T> { 
    pub fn new() -> Self { Self(scc::Bag::new()) }
    pub fn pop(&self) -> Option<T> { self.0.pop() }
    pub fn push(&self, t: T)  { self.0.push(t) }
}

pub struct BagIterator<T> { 
    bag : Bag<T>
}

impl<T> Iterator for BagIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.bag.pop()
    }
}

impl<T> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = BagIterator<T>;

    fn into_iter(self) -> BagIterator<T> {
        BagIterator { bag : self }
    }
}