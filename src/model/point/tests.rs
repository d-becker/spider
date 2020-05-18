use super::*;

#[test]
fn add_points() {
    let a = Point::new(2, -4);
    let b = Point::new(10, -5);

    let exp = Point::new(12, -9);
    let res = a.add(b);

    assert_eq!(exp, res);
}

#[test]
fn dir_to_point() {
    let directions = vec![
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
        Direction::NONE,
    ];

    let exp = vec![
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(-1, 0),
        Point::new(1, 0),
        Point::new(0, 0),
    ];

    let res: Vec<Point> = directions.iter().map(Direction::to_point).collect();
    assert_eq!(exp, res);
}
