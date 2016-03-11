extern crate gprs;

use gprs::heap::make_heap;
use gprs::heap::sort_heap;
use gprs::heap::push_heap;

#[test]
fn test_make_heap() {
    let mut v = vec![9, 7, 6, 8, 5, 3, 4, 1, 2];
    make_heap(&mut v[..]);
    assert_eq!(v, vec![9, 8, 6, 7, 5, 3, 4, 1, 2]);
}

#[test]
fn test_sort_heap() {
    let mut v = vec![9, 7, 6, 8, 5, 3, 4, 1, 2, 3, 7];
    make_heap(&mut v[..]);
    sort_heap(&mut v[..]);
    assert_eq!(v, vec![1, 2, 3, 3, 4, 5, 6, 7, 7, 8, 9]);
}

#[test]
fn test_push_heap() {
    let mut v = vec![1];
    push_heap(&mut v);
    assert_eq!(v, vec![1]);

    v.push(2);
    push_heap(&mut v);
    assert_eq!(v, vec![2, 1]);

    v.push(3);
    push_heap(&mut v);
    assert_eq!(v, vec![3, 1, 2]);

    v.push(4);
    push_heap(&mut v);
    assert_eq!(v, vec![4, 3, 2, 1]);

    v.push(5);
    push_heap(&mut v);
    assert_eq!(v, vec![5, 4, 2, 1, 3]);
}
