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

/// 用户提供自定义比函数时的堆操作
pub struct HeapComparator<'a, T : 'a> {
    sequence : &'a mut [T],                     // 被操作的slice
    comparator : &'a Fn(&T, &T) -> Ordering     // 用户自定义的比较函数
}

impl<'a, T : Ord + 'a> HeapComparator<'a, T> {
    /// 将一个无序的slice构造成堆
    pub fn make_heap(&mut self) {
        make_heap(self.sequence, self.comparator);
    }

    /// 将一个slice构造成堆 
    /// 
    /// 该slice必须满足除最后一个元素外其他元素构成一个堆
    /// # 用例
    /// ```
    /// use gprs::heap::HeapOperation;
    /// 
    /// let comp = |x : &i32, y : &i32| { y.cmp(x) };   // 最小堆
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

    /// 将堆根部元素移动到slice末尾, 并将剩下元素构造成堆
    /// # 用例
    /// ```
    /// use gprs::heap::HeapOperation;
    ///
    /// let comp = |x : &i32, y : &i32| { y.cmp(x) };   // 最小堆
    /// 
    /// let mut v = vec![1, 2, 3, 4, 5];
    /// v.compare_by(&comp).pop_heap();
    /// assert_eq!(v, vec![2, 4, 3, 5, 1]);             // 堆根部元素被移动到了slice末尾
    /// ```
    pub fn pop_heap(&mut self) {
        pop_heap(self.sequence, self.comparator);
    }

    /// 对已构成堆的slice进行排序
    /// # 用例
    /// ```
    /// use gprs::heap::HeapOperation;
    ///
    /// let comp = |x : &i32, y : &i32| { y.cmp(x) };   // 最小堆
    ///
    /// let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// v.compare_by(&comp).make_heap();                // 先构造堆
    /// v.compare_by(&comp).sort_heap();                // 再排序
    /// assert_eq!(v, vec![9, 6, 5, 5, 4, 3, 3, 2, 1, 1]);
    /// ```
    pub fn sort_heap(&mut self) {
        sort_heap(self.sequence, self.comparator);
    }
}

pub trait HeapOperation<'a, T> {
    /// 提供用户自己的比较函数以选择构造最大堆/最小堆
    fn compare_by(&'a mut self, &'a Fn(&T, &T) -> Ordering) -> HeapComparator<'a, T>;
    
    /// 直接构造堆(默认为最大堆)
    /// # 用例
    /// ```
    /// use gprs::heap::HeapOperation;
    ///
    /// let mut v = vec![1, 3, 4, 5, 2];
    /// v.make_heap();
    /// assert_eq!(v, vec![5, 3, 4, 1, 2]);
    /// ```
    fn make_heap(&mut self);

    /// 对堆进行排序(默认为最大堆)
    /// # 用例
    /// ```
    /// use gprs::heap::HeapOperation;
    ///
    /// let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// v.make_heap();      // 先构造堆
    /// v.sort_heap();      // 然后才能排序
    /// assert_eq!(v, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
    /// ```
    fn sort_heap(&mut self);

    /// 将一个末尾加入一个新元素的堆重新构造成堆
    /// # 用例
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
    fn push_heap(&mut self);

    /// 将堆根部元素移动到slice末尾, 并将剩下元素构造成堆
    /// # 用例
    /// ```
    /// use gprs::heap::HeapOperation;
    ///
    /// let mut v = vec![5, 4, 3, 2, 1];
    /// v.pop_heap();
    /// assert_eq!(v, vec![4, 2, 3, 1, 5]);
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
