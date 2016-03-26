/// 为slice提供旋转操作 
pub trait Rotatable<T> {
    /// 交换[0, mid), [mid, len)
    /// # 用例
    /// ```
    /// use gprs::rotate::Rotatable;
    ///
    /// let mut v = vec![1, 2, 3, 4, 5];
    /// v.rotate(3);
    /// assert_eq!(v, vec![4, 5, 1, 2, 3]);
    /// ```
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
