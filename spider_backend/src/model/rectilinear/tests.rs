use super::*;

#[test]
fn horizontal_ok() {
    let p1 = Point::new(2, 1);
    let p2 = Point::new(3, 1);

    assert!(horizontal(&p1, &p2));
}

#[test]
fn horizontal_false() {
    let p1 = Point::new(2, 2);
    let p2 = Point::new(3, 1);

    assert!(!horizontal(&p1, &p2));
}

#[test]
fn vertical_ok() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(1, 3);

    assert!(vertical(&p1, &p2));
}

#[test]
fn vertical_false() {
    let p1 = Point::new(2, 2);
    let p2 = Point::new(3, 1);

    assert!(!vertical(&p1, &p2));
}
