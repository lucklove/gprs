use std::mem;

fn swap_cell<T : Clone>(seq : &mut [T], l_index : usize, r_index : usize) {
    let mut tmp = seq[l_index].clone();
    mem::swap(&mut tmp, &mut seq[r_index]);
    mem::swap(&mut tmp, &mut seq[l_index]);
}

fn calculate_permutation<T : Ord + Clone>(seq : &mut [T], f : &Fn(&T, &T) -> bool) -> bool {
    if seq.len() < 2 {
        return false;
    }

    let mut back_index = seq.len() - 1;
    loop {
        let next_back_index = back_index;
        back_index -= 1;

        if f(&seq[back_index], &seq[next_back_index]) {
            let mut dest_index = seq.len() - 1;
            while !f(&seq[back_index], &seq[dest_index]) {
                dest_index -= 1;
            }

            swap_cell(seq, back_index, dest_index);
            seq[next_back_index..].reverse();
            return true;
        }

        if back_index == 0 {
            seq.reverse();
            return false;
        }
    }
}

pub fn prev_permutation<T : Ord + Clone>(seq : &mut [T]) -> bool {
    calculate_permutation(seq, &|l : &T, r : &T| { l > r })
}

pub fn next_permutation<T : Ord + Clone>(seq : &mut [T]) -> bool {
    calculate_permutation(seq, &|l : &T, r : &T| { l < r })
}
