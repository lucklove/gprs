use std::cmp::Ordering;

#[cfg(test)]
mod test {
    #[test]
    fn test_fix_heap() {
        let mut v = vec![1, 2, 3];
        super::fix_heap(&mut v[..], 0, &super::default_comparator);
        assert_eq!(v, vec![3, 2, 1]);

        let mut v = vec![1, 4, 3, 6, 5];
        super::fix_heap(&mut v[..], 0, &super::default_comparator);
        assert_eq!(v, vec![4, 6, 3, 1, 5]);
    }

    #[test]
    fn test_make_heap() {
        let mut v = vec![9, 7, 6, 8, 5, 3, 4, 1, 2];
        super::make_heap(&mut v[..], &super::default_comparator);
        assert_eq!(v, vec![9, 8, 6, 7, 5, 3, 4, 1, 2]);
    }

    #[test]
    fn test_sort_heap() {
        let mut v = vec![9, 7, 6, 8, 5, 3, 4, 1, 2, 3, 7];
        super::make_heap(&mut v[..], &super::default_comparator);
        super::sort_heap(&mut v[..], &super::default_comparator);
        assert_eq!(v, vec![1, 2, 3, 3, 4, 5, 6, 7, 7, 8, 9]);
    }

    #[test]
    fn test_push_heap() {
        let mut v = vec![1];
        super::push_heap(&mut v, &super::default_comparator);
        assert_eq!(v, vec![1]);

        v.push(2);
        super::push_heap(&mut v, &super::default_comparator);
        assert_eq!(v, vec![2, 1]);

        v.push(3);
        super::push_heap(&mut v, &super::default_comparator);
        assert_eq!(v, vec![3, 1, 2]);

        v.push(4);
        super::push_heap(&mut v, &super::default_comparator);
        assert_eq!(v, vec![4, 3, 2, 1]);

        v.push(5);
        super::push_heap(&mut v, &super::default_comparator);
        assert_eq!(v, vec![5, 4, 2, 1, 3]);
    }

    #[test]
    fn test_pop_heap() {
        let mut v = vec![5, 4, 2, 1, 3];

        super::pop_heap(&mut v, &super::default_comparator);
        assert_eq!(v, vec![4, 3, 2, 1, 5]);
        v.remove(4);

        super::pop_heap(&mut v, &super::default_comparator);
        assert_eq!(v, vec![3, 1, 2, 4]);    
        v.remove(3);
    }
}

fn default_comparator<T : Ord>(left : &T, right : &T) -> Ordering {
    left.cmp(right)
}

fn fix_heap<T>(seq : &mut [T], index : usize, comp : &Fn(&T, &T) -> Ordering) {
    let len = seq.len();
    let left_child_index = (index + 1) * 2 - 1;
    let right_child_index = (index + 1) * 2;
    if left_child_index < len && comp(&seq[index], &seq[left_child_index]) == Ordering::Less
        && right_child_index < len && comp(&seq[index], &seq[right_child_index]) == Ordering::Less {
        if comp(&seq[right_child_index], &seq[left_child_index]) == Ordering::Less {
            seq.swap(index, left_child_index);
            fix_heap(&mut seq[..], left_child_index, comp);
        } else {
            seq.swap(index, right_child_index);
            fix_heap(&mut seq[..], right_child_index, comp);
        }
    } else if left_child_index < len && comp(&seq[index], &seq[left_child_index]) == Ordering::Less {
        seq.swap(index, left_child_index);
        fix_heap(&mut seq[..], left_child_index, comp);
    } else if right_child_index < len && comp(&seq[index], &seq[right_child_index]) == Ordering::Less {
        seq.swap(index, right_child_index);
        fix_heap(&mut seq[..], right_child_index, comp);
    }    
}

fn push_heap<T : Ord>(seq : &mut [T], comp : &Fn(&T, &T) -> Ordering) {
    let len = seq.len();
    if len < 2 {
        return;
    }

    let father_index = len / 2 - 1;
    if comp(&seq[father_index], &seq[len-1]) == Ordering::Less {
        seq.swap(len - 1, father_index);
        push_heap(&mut seq[0..father_index+1], comp);
    }
}

fn pop_heap<T : Ord>(seq : &mut [T], comp : &Fn(&T, &T) -> Ordering) {
    let len = seq.len();
    if len < 2 {
        return;
    }

    seq.swap(0, len - 1);
    fix_heap(&mut seq[0..len-1], 0, comp);
}

fn make_heap<T : Ord>(seq : &mut [T], comp : &Fn(&T, &T) -> Ordering) {
    if seq.len() < 2 {
        return;
    }

    let mut index = seq.len();
    loop {
        index -= 1;
        fix_heap(&mut seq[..], index, comp);
        if index == 0 {
            break;
        }
    }
}

fn sort_heap<T : Ord>(seq : &mut [T], comp : &Fn(&T, &T) -> Ordering) {
    let len = seq.len();
    if len < 2 {
        return;
    }

    for index in 0..len {
        seq.swap(0, len - index - 1);
        fix_heap(&mut seq[0..len-index-1], 0, comp);
    }
}

/// Operations when user version comparator was present
pub struct HeapComparator<'a, T : 'a> {
    sequence : &'a mut [T],                     // slice to be operated
    comparator : &'a Fn(&T, &T) -> Ordering     // user version comparator
}

impl<'a, T : Ord + 'a> HeapComparator<'a, T> {
    /// make heap from unordered slice
    /// # Examples
    /// basic usage:
    ///
    /// ```
    /// use gprs::heap::HeapOperation;
    ///
    /// let comp = |x : &i32, y : &i32| { y.cmp(x) };   // for minimum heap
    /// let mut v = vec![5, 4, 3, 2, 1];
    /// v.compare_by(&comp).make_heap();
    /// assert_eq!(v, vec![1, 2, 3, 5, 4]);
    /// ```
    pub fn make_heap(&mut self) {
        make_heap(self.sequence, self.comparator);
    }

    /// push a new item to a heap
    /// # Examples
    /// basic usage:
    ///
    /// ```
    /// use gprs::heap::HeapOperation;
    /// 
    /// let comp = |x : &i32, y : &i32| { y.cmp(x) };   // for minimum heap
    ///
    /// let mut v = vec![5];
    /// 
    /// v.push(4);
    /// v.compare_by(&comp).push_heap();
    /// 
    /// v.push(3);
    /// v.compare_by(&comp).push_heap();
    ///
    /// v.push(2);
    /// v.compare_by(&comp).push_heap();
    ///
    /// v.push(1);
    /// v.compare_by(&comp).push_heap();
    /// assert_eq!(v, vec![1, 2, 4, 5, 3]);
    /// ```
    pub fn push_heap(&mut self) {
        push_heap(self.sequence, self.comparator);
    }

    /// pop an item from heap
    /// # Examples
    /// basic usage:
    ///
    /// ```
    /// use gprs::heap::HeapOperation;
    ///
    /// let comp = |x : &i32, y : &i32| { y.cmp(x) };   // for minimum heap
    /// 
    /// let mut v = vec![1, 2, 3, 4, 5];
    /// v.compare_by(&comp).pop_heap();
    /// assert_eq!(v, vec![2, 4, 3, 5, 1]);             // dest item show at the end of slice
    /// ```
    pub fn pop_heap(&mut self) {
        pop_heap(self.sequence, self.comparator);
    }

    /// sort a heap
    /// # Examples
    /// basic usage:  
    ///
    /// ```
    /// use gprs::heap::HeapOperation;
    ///
    /// let comp = |x : &i32, y : &i32| { y.cmp(x) };   // for minimum heap
    ///
    /// let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// v.compare_by(&comp).make_heap();                // construct heap first
    /// v.compare_by(&comp).sort_heap();                // sort second
    /// assert_eq!(v, vec![9, 6, 5, 5, 4, 3, 3, 2, 1, 1]);
    /// ```
    pub fn sort_heap(&mut self) {
        sort_heap(self.sequence, self.comparator);
    }
}

/// Heap operations by default
/// # Examples
/// construct maximum heap:
///
/// ```
/// use gprs::heap::HeapOperation;
///
/// let mut v = vec![1, 3, 4, 5, 2];
/// v.make_heap();
/// assert_eq!(v, vec![5, 3, 4, 1, 2]);
/// ```
/// sort a maximum heap:
///
/// ```
/// use gprs::heap::HeapOperation;
///
/// let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
/// v.make_heap();      // construct heap first
/// v.sort_heap();      // then sort heap
/// assert_eq!(v, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
/// ```
/// push a new item to a heap:
///
/// ```
/// use gprs::heap::HeapOperation;
///
/// let mut v = vec![1];
/// v.push(2);
/// v.push_heap();
/// v.push(3);
/// v.push_heap();
/// v.push(4);
/// v.push_heap();
/// v.push(5);
/// v.push_heap();
/// assert_eq!(v, vec![5, 4, 2, 1, 3]);
/// ```
/// pop an item from a heap:
///
/// ```
/// use gprs::heap::HeapOperation;
///
/// let mut v = vec![5, 4, 3, 2, 1];
/// v.pop_heap();
/// assert_eq!(v, vec![4, 2, 3, 1, 5]);     // dest item show at the end of slice
/// ```
pub trait HeapOperation<'a, T> {
    /// let user choice to construct neither minimum heap or maximum heap
    fn compare_by(&'a mut self, &'a Fn(&T, &T) -> Ordering) -> HeapComparator<'a, T>;
    
    /// construct maximum heap
    fn make_heap(&mut self);

    /// sort a maximum heap
    fn sort_heap(&mut self);

    /// push a new item to a heap
    fn push_heap(&mut self);

    /// pop an item from a heap
    fn pop_heap(&mut self);
}

impl<'a, T : Ord + 'a> HeapOperation<'a, T> for [T] {
    fn compare_by(&'a mut self, comp : &'a Fn(&T, &T) -> Ordering) -> HeapComparator<'a, T> {
        HeapComparator{ sequence : self, comparator : comp }
    }

    fn make_heap(&mut self) {
        make_heap(self, &default_comparator);
    }

    fn push_heap(&mut self) {
        push_heap(self, &default_comparator);
    }

    fn pop_heap(&mut self) {
        pop_heap(self, &default_comparator);
    }

    fn sort_heap(&mut self) {
        sort_heap(self, &default_comparator);
    }
}
