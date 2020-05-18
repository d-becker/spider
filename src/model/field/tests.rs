use super::*;

#[test]
fn area_rectangle() {
    let points = vec![
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(0, 10),
    ];

    let exp = 200;
    let res = shoelace_poly_area(&points);
    assert_eq!(exp, res);
}

#[test]
fn area_polygon() {
    let points = vec![
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];

    let exp = 200 + 100;
    let res = shoelace_poly_area(&points);
    assert_eq!(exp, res);
}
