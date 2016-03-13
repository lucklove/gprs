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

pub struct HeapComparator<'a, T : 'a> {
    sequence : &'a mut [T],
    comparator : &'a Fn(&T, &T) -> Ordering
}

impl<'a, T : Ord + 'a> HeapComparator<'a, T> {
    pub fn make_heap(&mut self) {
        make_heap(self.sequence, self.comparator);
    }

    pub fn push_heap(&mut self) {
        push_heap(self.sequence, self.comparator);
    }

    pub fn pop_heap(&mut self) {
        pop_heap(self.sequence, self.comparator);
    }

    pub fn sort_heap(&mut self) {
        sort_heap(self.sequence, self.comparator);
    }
}

pub trait HeapOperation<'a, T> {
    fn compare_by(&'a mut self, &'a Fn(&T, &T) -> Ordering) -> HeapComparator<'a, T>;
    fn make_heap(&mut self);
    fn sort_heap(&mut self);
    fn push_heap(&mut self);
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
