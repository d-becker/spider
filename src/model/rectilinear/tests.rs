use super::*;

#[test]
fn test_collinear_vertical() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(1, 3);
    let p3 = Point::new(1, 4);

    assert!(collinear(p1, p2, p3));
}

#[test]
fn test_collinear_horizontal() {
    let p1 = Point::new(2, 1);
    let p2 = Point::new(3, 1);
    let p3 = Point::new(4, 1);

    assert!(collinear(p1, p2, p3));
}

#[test]
fn test_collinear_false() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(1, 3);
    let p3 = Point::new(2, 4);

    assert!(!collinear(p1, p2, p3));
}

#[test]
fn test_line_contains_inside() {
    let start = Point::new(2, 1);
    let end = Point::new(4, 1);
    let p = Point::new(3, 1);

    assert!(line_contains(start, end, p));
}

#[test]
fn test_line_contains_on_edge() {
    let start = Point::new(2, 1);
    let end = Point::new(4, 1);
    let p = end;

    assert!(line_contains(start, end, p));
}

#[test]
fn test_line_contains_inside_vertical_reversed() {
    let start = Point::new(1, 4);
    let end = Point::new(1, 2);
    let p = Point::new(1, 3);

    assert!(line_contains(start, end, p));
}

#[test]
fn test_line_contains_inside_horizontal_reversed() {
    let start = Point::new(4, 1);
    let end = Point::new(2, 1);
    let p = Point::new(3, 1);

    assert!(line_contains(start, end, p));
}

fn get_path<'a>(points: &[Point]) -> Path {
    let mut path = Path::new();

    for point in points {
        path.add(*point).unwrap();
    }

    path
}

#[test]
fn path_no_motion() {
    let point = Point::new(-1, 0);
    let mut path = Path::with_start(point);

    let old_points = path.points().to_vec();

    let res = path.add(point);
    assert!(res.is_ok());

    let new_points = path.points();
    assert_eq!(old_points, new_points);
}

#[test]
fn path_simple_motion() {
    let mut path = Path::new();
    let new_point = Point::new(-1, 0);

    let expected =  {
        let mut old_points = path.points().to_vec();
        old_points.push(new_point);
        old_points
    };

    let res = path.add(new_point);
    assert!(res.is_ok());

    let new_points = path.points();
    assert_eq!(expected, new_points);
}

#[test]
fn path_collinear_motion() {
    let old_points = &[Point::new(0, 0), Point::new(1, 0)];
    let mut path = get_path(old_points);

    let new_point = Point::new(2, 0);

    path.add(new_point).unwrap();

    let expected = {
        let mut temp = old_points.to_vec();
        *temp.last_mut().unwrap() = new_point;
        temp
    };

    let new_points = path.points();

    assert_eq!(expected, new_points);
}

#[test]
fn spider_loop_resolved() {
    let points = [
        Point::origin(),
        Point::new(10, 0),
        Point::new(10, 1),
        Point::new(8, 1),
    ];

    let mut path = get_path(&points);
    let new_point = Point::new(8, 0);
    path.add(new_point).unwrap();

    let exp = [Point::origin(), Point::new(8, 0)];
    let new_points = path.points();

    assert_eq!(exp, new_points);
}
