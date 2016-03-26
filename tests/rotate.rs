extern crate gprs;

use gprs::rotate::Rotatable;

#[test]
fn test_rotate() {
    let mut v = vec![0, 1, 2, 3, 4, 5];
    v.rotate(3);
    assert_eq!(v, vec![3, 4, 5, 0, 1, 2]);
    v.rotate(5);
    assert_eq!(v, vec![2, 3, 4, 5, 0, 1]);
    v.rotate(1);
    assert_eq!(v, vec![3, 4, 5, 0, 1, 2]);
    v.rotate(0);
    assert_eq!(v, vec![3, 4, 5, 0, 1, 2]);
    v.rotate(6);
    assert_eq!(v, vec![3, 4, 5, 0, 1, 2]);
}
