pub fn compute(vec: &mut Vec<u64>) -> u64 {
    let mut d = vec[0];
    for m in &vec[1..] {
        d = gcd(d, *m);
    }
    d
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

#[test]
#[should_panic]
fn test_gcd_zero_params() {
    gcd(0, 1);
    gcd(1, 0);
    gcd(0, 0);
}

#[test]
fn test_compute() {
    let vec = &mut vec![42, 56];
    assert_eq!(compute(vec), 14);

    let vec = &mut vec![799459, 28823, 27347];
    assert_eq!(compute(vec), 41);

    let vec = &mut vec![83];
    assert_eq!(compute(vec), 83);

    let vec = &mut vec![0];
    assert_eq!(compute(vec), 0);
}
