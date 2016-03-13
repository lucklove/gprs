extern crate gprs;

use gprs::heap::HeapOperation;

#[test]
fn test_make_heap() {
    let mut v = vec![1, 3, 4, 5, 2];

    //最小堆
    v.compare_by(&|x : &i32, y : &i32| { y.cmp(x) }).make_heap();
    assert_eq!(v, vec![1, 2, 4, 5, 3]);

    //默认为最大堆
    v.make_heap();
    assert_eq!(v, vec![5, 3, 4, 2, 1]);
}

#[test]
fn test_sort_heap() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    //从大到小排列
    let comp = |x : &i32, y : &i32| { y.cmp(x) };
    v.compare_by(&comp).make_heap();
    v.compare_by(&comp).sort_heap();
    assert_eq!(v, vec![9, 6, 5, 5, 4, 3, 3, 2, 1, 1]);     

    //默认从小到大排列
    v.make_heap();
    v.sort_heap();
    assert_eq!(v, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);     
}

#[test]
fn test_push_heap() {
    //最小堆
    let comp = |x : &i32, y : &i32| { y.cmp(x) };

    let mut v = vec![5];
    v.compare_by(&comp).push_heap();
    assert_eq!(v, vec![5]);

    v.push(4);
    v.compare_by(&comp).push_heap();
    assert_eq!(v, vec![4, 5]);

    v.push(3);
    v.compare_by(&comp).push_heap();
    assert_eq!(v, vec![3, 5, 4]);

    v.push(2);
    v.compare_by(&comp).push_heap();
    assert_eq!(v, vec![2, 3, 4, 5]);

    v.push(1);
    v.compare_by(&comp).push_heap();
    assert_eq!(v, vec![1, 2, 4, 5, 3]);
    
    //默认为最大堆
    v = vec![1];
    v.push_heap();
    assert_eq!(v, vec![1]);

    v.push(2);
    v.push_heap();
    assert_eq!(v, vec![2, 1]);

    v.push(3);
    v.push_heap();
    assert_eq!(v, vec![3, 1, 2]);

    v.push(4);
    v.push_heap();
    assert_eq!(v, vec![4, 3, 2, 1]);

    v.push(5);
    v.push_heap();
    assert_eq!(v, vec![5, 4, 2, 1, 3]);
}

#[test]
fn test_pop_heap() {
    let comp = |x : &i32, y : &i32| { y.cmp(x) };
    
    let mut v = vec![1, 2, 3, 4, 5];

    v[0..5].compare_by(&comp).pop_heap();
    assert_eq!(v, vec![2, 4, 3, 5, 1]);

    v[0..4].compare_by(&comp).pop_heap();
    assert_eq!(v, vec![3, 4, 5, 2, 1]);

    v[0..3].compare_by(&comp).pop_heap();
    assert_eq!(v, vec![4, 5, 3, 2, 1]);

    v[0..2].compare_by(&comp).pop_heap();
    assert_eq!(v, vec![5, 4, 3, 2, 1]);

    v[0..1].compare_by(&comp).pop_heap();
    assert_eq!(v, vec![5, 4, 3, 2, 1]);
}
