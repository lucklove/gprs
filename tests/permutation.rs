extern crate gprs;

use gprs::permutation::next_permutation;
use gprs::permutation::prev_permutation;

#[test]
fn test_prev_permutation() {
    let mut seq = vec![3, 2, 1];

    assert_eq!(prev_permutation(&mut seq), true);
    assert_eq!(seq, [3, 1, 2]);

    assert_eq!(prev_permutation(&mut seq), true);
    assert_eq!(seq, [2, 3, 1]);

    assert_eq!(prev_permutation(&mut seq), true);
    assert_eq!(seq, [2, 1, 3]);

    assert_eq!(prev_permutation(&mut seq), true);
    assert_eq!(seq, [1, 3, 2]);

    assert_eq!(prev_permutation(&mut seq), true);
    assert_eq!(seq, [1, 2, 3]);

    assert_eq!(prev_permutation(&mut seq), false);
    assert_eq!(seq, [3, 2, 1]);
}

#[test]
fn test_next_permutation() {
    let mut seq = vec![1, 2, 2];

    assert_eq!(next_permutation(&mut seq), true);
    assert_eq!(seq, [2, 1, 2]);

    assert_eq!(next_permutation(&mut seq), true);
    assert_eq!(seq, [2, 2, 1]);

    assert_eq!(next_permutation(&mut seq), false);
    assert_eq!(seq, [1, 2, 2]);
}
