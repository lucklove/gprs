extern crate gprs;

use gprs::heap::make_heap;
use gprs::heap::heap_sort;

#[test]
fn test_make_heap() {
    let mut v = vec![9, 7, 6, 8, 5, 3, 4, 1, 2];
    make_heap(&mut v[..], &|l, r| { l < r });
    assert_eq!(v, vec![1, 2, 3, 7, 5, 6, 4, 8, 9]);
}

#[test]
fn test_heap_sort() {
    let mut v = vec![9, 7, 6, 8, 5, 3, 4, 1, 2, 3, 7];
    make_heap(&mut v[..], &|l, r| { l < r });
    heap_sort(&mut v[..], &|l, r| { l < r });
    assert_eq!(v, vec![1, 2, 3, 3, 4, 5, 6, 7, 7, 8, 9]);
    make_heap(&mut v[..], &|l, r| { l > r });
    heap_sort(&mut v[..], &|l, r| { l > r });
    assert_eq!(v, vec![9, 8, 7, 7, 6, 5, 4, 3, 3, 2, 1]);
}
