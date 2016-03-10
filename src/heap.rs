#[cfg(test)]
mod test {
    #[test]
    fn test_fix_heap() {
        let mut v = vec![3, 1, 2];
        super::fix_heap(&mut v[..], 0, &|l, r| { l < r });
        assert_eq!(v, vec![1, 3, 2]);

        let mut v = vec![5, 3, 6, 4, 1];
        super::fix_heap(&mut v[..], 0, &|l, r| { l < r });
        assert_eq!(v, vec![3, 1, 6, 4, 5]);
    }
}

fn fix_heap<T>(seq : &mut [T], index : usize, comparator : &Fn(&T, &T) -> bool) {
    let len = seq.len();
    let left_child_index = (index + 1) * 2 - 1;
    let right_child_index = (index + 1) * 2;
    if left_child_index < len && !comparator(&seq[index], &seq[left_child_index]) 
        && right_child_index < len && !comparator(&seq[index], &seq[right_child_index]) {
        if !comparator(&seq[right_child_index], &seq[left_child_index]) {
            seq.swap(index, left_child_index);
            fix_heap(&mut seq[..], left_child_index, comparator);
        } else {
            seq.swap(index, right_child_index);
            fix_heap(&mut seq[..], right_child_index, comparator);
        }
    } else if left_child_index < len && !comparator(&seq[index], &seq[left_child_index]) {
        seq.swap(index, left_child_index);
        fix_heap(&mut seq[..], left_child_index, comparator);
    } else if right_child_index < len && !comparator(&seq[index], &seq[right_child_index]) {
        seq.swap(index, right_child_index);
        fix_heap(&mut seq[..], right_child_index, comparator);
    }    
}

pub fn make_heap<T>(seq : &mut [T], comparator : &Fn(&T, &T) -> bool) {
    if seq.len() < 2 {
        return;
    }

    let mut index = seq.len();
    loop {
        index -= 1;
        fix_heap(&mut seq[..], index, comparator);
        if index == 0 {
            break;
        }
    }
}

pub fn heap_sort<T>(seq : &mut [T], comparator : &Fn(&T, &T) -> bool) {
    let len = seq.len();
    if len < 2 {
        return;
    }

    for index in 0..len {
        seq.swap(0, len - index - 1);
        fix_heap(&mut seq[0..len-index-1], 0, comparator);
    }

    seq.reverse();
}
