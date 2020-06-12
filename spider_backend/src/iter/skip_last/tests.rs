use super::*;

#[test]
fn forward() {
    let start = 1;
    let end = 5;
    let orig = start..end;
    let skip_last = SkipLast::new(orig);

    let expected = (start..end - 1).collect::<Vec<_>>();

    assert_eq!(expected, skip_last.collect::<Vec<_>>());
}

#[test]
fn forward_zero_length() {
    let orig = 0..0;
    let skip_last = SkipLast::new(orig);

    let expected = (0..0).collect::<Vec<_>>();

    assert_eq!(expected, skip_last.collect::<Vec<_>>());
}

#[test]
fn clone() {
    let mut orig = SkipLast::new(1..5);

    orig.next();

    let cloned = orig.clone();

    assert!(orig.eq(cloned));
}

#[test]
fn exact_size() {
    let orig = 1..5;
    let orig_size = orig.len();

    let skip_last = SkipLast::new(orig);
    assert_eq!(orig_size - 1, skip_last.len());
}

#[test]
fn exact_size_zero() {
    let orig = 0..0;
    let orig_size = orig.len();
    assert_eq!(0, orig_size);

    let skip_last = SkipLast::new(orig);
    assert_eq!(orig_size, skip_last.len());
}

#[test]
fn reverse() {
    let start = 1;
    let end = 5;
    let orig = start..end;
    let skip_last = SkipLast::new(orig.clone());
    let reversed = skip_last.rev().collect::<Vec<_>>();
    let expected = (start..end - 1).rev().collect::<Vec<_>>();

    assert_eq!(expected, reversed);
}

#[test]
fn reverse_zero() {
    let orig = 0..0;
    let skip_last = SkipLast::new(orig.clone());
    let reversed = skip_last.rev().collect::<Vec<_>>();
    let expected = orig.collect::<Vec<_>>();

    assert_eq!(expected, reversed);
}
