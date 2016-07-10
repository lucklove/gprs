extern crate gprs;

use gprs::scope_guard::ScopeGuard;

#[test]
fn test_rotate() {
    let mut c = 0;

    for _ in 0..9 {
        let _ = ScopeGuard::new(||{
            c += 1;
        });
    }

    assert_eq!(c, 9);
}
