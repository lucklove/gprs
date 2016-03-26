fn calculate_permutation<T : Ord>(seq : &mut [T], comparator : &Fn(&T, &T) -> bool) -> bool {
    if seq.len() < 2 {
        return false;
    }

    let mut back_index = seq.len() - 1;
    loop {
        let next_back_index = back_index;
        back_index -= 1;

        if comparator(&seq[back_index], &seq[next_back_index]) {
            let mut dest_index = seq.len() - 1;
            while !comparator(&seq[back_index], &seq[dest_index]) {
                dest_index -= 1;
            }

            seq.swap(back_index, dest_index);
            seq[next_back_index..].reverse();
            return true;
        }

        if back_index == 0 {
            seq.reverse();
            return false;
        }
    }
}

/// 求slice内元素的排列组合
pub trait Permutation<T> {
    /// 上一个组合, 若没有更多的组合则返回false
    /// # 用例
    /// ```
    /// use gprs::permutation::Permutation;
    ///
    /// let mut seq = vec![3, 2, 1];
    /// assert_eq!(seq.prev_permutation(), true);
    /// assert_eq!(seq, [3, 1, 2]);
    /// ```
    fn prev_permutation(&mut self) -> bool;

    /// 下一个组合, 若没有更多的组合则返回false
    /// # 用例
    /// ```
    /// use gprs::permutation::Permutation;
    ///
    /// let mut seq = vec![1, 2, 2];
    /// assert_eq!(seq.next_permutation(), true);
    /// assert_eq!(seq, [2, 1, 2]);
    /// ```
    fn next_permutation(&mut self) -> bool;
}

impl<T : Ord> Permutation<T> for [T] {
    fn prev_permutation(&mut self) -> bool {
        calculate_permutation(self, &|l : &T, r : &T| { l > r })
    }

    fn next_permutation(&mut self) -> bool {
        calculate_permutation(self, &|l : &T, r : &T| { l < r })
    }
}
