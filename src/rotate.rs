/// Rotate slice 
/// # Examples
/// basic usage:
///
/// ```
/// use gprs::rotate::Rotatable;
///
/// let mut v = vec![1, 2, 3, 4, 5];
/// v.rotate(3);
/// assert_eq!(v, vec![4, 5, 1, 2, 3]);
/// ```
pub trait Rotatable<T> {
    /// swap self[0..mid] and self[mid, self.len()]
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
