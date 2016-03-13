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

pub trait Permutation<T> {
    fn prev_permutation(&mut self) -> bool;
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
