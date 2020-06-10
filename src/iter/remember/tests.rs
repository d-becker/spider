use super::*;

#[test]
fn many_elements() {
    let elements: Vec<i32> = (0_i32..10_i32).collect();
    let mut remember = Remember::new(elements.iter().copied(), 2, 3);

    let mut collected = Vec::new();
    while let Some(elem) = remember.next() {
        collected.push(elem);
    }

    assert_eq!(elements, collected);

    let (front, back) = remember.get_remembered();
    assert_eq!(&elements[0..2], &*front);
    assert_eq!(&elements[7..], &*back);
}

#[test]
fn overlapping_front_back() {
    let elements: Vec<i32> = (0_i32..4_i32).collect();
    let mut remember = Remember::new(elements.iter().copied(), 2, 3);

    let mut collected = Vec::new();
    while let Some(elem) = remember.next() {
        collected.push(elem);
    }

    assert_eq!(elements, collected);

    let (front, back) = remember.get_remembered();
    assert_eq!(&elements[0..2], &*front);
    assert_eq!(&elements[1..], &*back);
}
