pub trait Rotatable<T> {
    fn rotate(&mut self, mid : usize);
}

impl<T> Rotatable<T> for [T] {
    fn rotate(&mut self, mid : usize) {
        let len = self.len();
        self[0..mid].reverse();
        self[mid..len].reverse();
        self.reverse();
    }
}
