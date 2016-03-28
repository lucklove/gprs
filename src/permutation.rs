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

/// Get all permutations from a slice
/// # Examples
/// get previous permutation:
///
/// ```
/// use gprs::permutation::Permutation;
///
/// let mut seq = vec![3, 2, 1];
/// assert_eq!(seq.prev_permutation(), true);
/// assert_eq!(seq, [3, 1, 2]);
/// ```
/// get next permutation:
///
/// ```
/// use gprs::permutation::Permutation;
///
/// let mut seq = vec![1, 2, 2];
/// assert_eq!(seq.next_permutation(), true);
/// assert_eq!(seq, [2, 1, 2]);
/// ```
pub trait Permutation<T> {
    /// get previous permutation, if there is no more, return false
    fn prev_permutation(&mut self) -> bool;

    /// get next permutation, if there is no more, return false
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
