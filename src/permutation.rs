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

pub fn prev_permutation<T : Ord>(seq : &mut [T]) -> bool {
    calculate_permutation(seq, &|l : &T, r : &T| { l > r })
}

pub fn next_permutation<T : Ord>(seq : &mut [T]) -> bool {
    calculate_permutation(seq, &|l : &T, r : &T| { l < r })
}
