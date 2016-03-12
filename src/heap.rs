#[cfg(test)]
mod test {
    #[test]
    fn test_fix_heap() {
        let mut v = vec![1, 2, 3];
        super::fix_heap(&mut v[..], 0);
        assert_eq!(v, vec![3, 2, 1]);

        let mut v = vec![1, 4, 3, 6, 5];
        super::fix_heap(&mut v[..], 0);
        assert_eq!(v, vec![4, 6, 3, 1, 5]);
    }
}

fn fix_heap<T : Ord>(seq : &mut [T], index : usize) {
    let len = seq.len();
    let left_child_index = (index + 1) * 2 - 1;
    let right_child_index = (index + 1) * 2;
    if left_child_index < len && seq[index] < seq[left_child_index] 
        && right_child_index < len && seq[index] < seq[right_child_index] {
        if seq[right_child_index] < seq[left_child_index] {
            seq.swap(index, left_child_index);
            fix_heap(&mut seq[..], left_child_index);
        } else {
            seq.swap(index, right_child_index);
            fix_heap(&mut seq[..], right_child_index);
        }
    } else if left_child_index < len && seq[index] < seq[left_child_index] {
        seq.swap(index, left_child_index);
        fix_heap(&mut seq[..], left_child_index);
    } else if right_child_index < len && seq[index] < seq[right_child_index] {
        seq.swap(index, right_child_index);
        fix_heap(&mut seq[..], right_child_index);
    }    
}

pub fn push_heap<T : Ord>(seq : &mut [T]) {
    let len = seq.len();
    if len < 2 {
        return;
    }

    let father_index = len / 2 - 1;
    println!("{}", len - 1);
    println!("{}", father_index);
    if seq[len-1] > seq[father_index] {
        seq.swap(len - 1, father_index);
        push_heap(&mut seq[0..father_index+1]);
    }
}

pub fn pop_heap<T : Ord>(seq : &mut [T]) {
    let len = seq.len();
    if len < 2 {
        return;
    }

    seq.swap(0, len - 1);
    fix_heap(&mut seq[0..len-1], 0);
}

pub fn make_heap<T : Ord>(seq : &mut [T]) {
    if seq.len() < 2 {
        return;
    }

    let mut index = seq.len();
    loop {
        index -= 1;
        fix_heap(&mut seq[..], index);
        if index == 0 {
            break;
        }
    }
}

pub fn sort_heap<T : Ord>(seq : &mut [T]) {
    let len = seq.len();
    if len < 2 {
        return;
    }

    for index in 0..len {
        seq.swap(0, len - index - 1);
        fix_heap(&mut seq[0..len-index-1], 0);
    }
}
