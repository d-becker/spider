use super::*;

#[test]
fn line_from_points_ok() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(1, 3);

    assert!(Line::from_points(p1, p2).is_some());
    assert!(Line::from_points(&p1, &p2).is_some());
}

#[test]
fn line_from_points_none() {
    let p1 = Point::new(2, 2);
    let p2 = Point::new(1, 3);

    assert!(Line::from_points(p1, p2).is_none());
    assert!(Line::from_points(&p1, &p2).is_none());
}

#[test]
fn line_directions() {
    let p0 = Point::new(0, 0);

    let line_left = Line::from_points(p0, Point::new(-10, 0)).unwrap();
    let line_right = Line::from_points(p0, Point::new(5, 0)).unwrap();
    let line_up = Line::from_points(p0, Point::new(0, -9)).unwrap();
    let line_down = Line::from_points(p0, Point::new(0, 3)).unwrap();

    assert_eq!(Direction::LEFT, line_left.direction());
    assert_eq!(Direction::RIGHT, line_right.direction());
    assert_eq!(Direction::UP, line_up.direction());
    assert_eq!(Direction::DOWN, line_down.direction());
}

#[test]
fn collinear_vertical() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(1, 3);
    let line = Line::from_points(p1, p2).unwrap();
    let p3 = Point::new(1, 4);

    assert!(line.collinear(p3));
}

#[test]
fn collinear_horizontal() {
    let p1 = Point::new(2, 1);
    let p2 = Point::new(3, 1);
    let line = Line::from_points(p1, p2).unwrap();

    let p3 = Point::new(4, 1);

    assert!(line.collinear(p3));
}

#[test]
fn collinear_false() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(1, 3);
    let line = Line::from_points(p1, p2).unwrap();

    let p3 = Point::new(2, 4);

    assert!(!line.collinear(p3));
}

#[test]
fn collinear_false_2() {
    let p1 = Point::new(0, 10);
    let p2 = Point::new(0, 0);
    let line = Line::from_points(p1, p2).unwrap();

    let p3 = Point::new(20, 10);

    assert!(!line.collinear(p3));
}

fn check_point_on_side(start: &Point, end: &Point, point: &Point, expected: i32) {
    let line = Line::from_points(start, end).unwrap();
    assert_eq!(expected, line.point_on_side(point));

    let opposite_line = Line::from_points(end, start).unwrap();
    assert_eq!(-expected, opposite_line.point_on_side(point));
}

#[test]
fn point_on_side() {
    let point = Point::new(5, 5);

    // Vertical.
    check_point_on_side(&Point::new(0, 0), &Point::new(0, 4), &point, 1);

    // Horizontal.
    check_point_on_side(&Point::new(0, 0), &Point::new(4, 0), &point, -1);

    // Collinear.
    check_point_on_side(&Point::new(5, 0), &Point::new(5, 3), &point, 0);
}

#[test]
fn line_contains_inside() {
    let start = Point::new(2, 1);
    let end = Point::new(4, 1);
    let line = Line::from_points(start, end).unwrap();

    let p = Point::new(3, 1);

    assert!(line.contains(p));
}

#[test]
fn line_contains_on_edge() {
    let start = Point::new(2, 1);
    let end = Point::new(4, 1);
    let line = Line::from_points(start, end).unwrap();

    let p = end;

    assert!(line.contains(p));
}

#[test]
fn line_contains_inside_vertical_reversed() {
    let start = Point::new(1, 4);
    let end = Point::new(1, 2);
    let line = Line::from_points(start, end).unwrap();

    let p = Point::new(1, 3);

    assert!(line.contains(p));
}

#[test]
fn line_contains_inside_horizontal_reversed() {
    let start = Point::new(4, 1);
    let end = Point::new(2, 1);
    let line = Line::from_points(start, end).unwrap();

    let p = Point::new(3, 1);

    assert!(line.contains(p));
}

fn check_lines_intersect(
    l1: (&Point, &Point),
    l2: (&Point, &Point),
    expected: Option<LineIntersection>,
) {
    let line1 = Line::from_points(l1.0, l1.1).unwrap();
    let line1_rev = Line::from_points(l1.1, l1.0).unwrap();

    let line2 = Line::from_points(l2.0, l2.1).unwrap();
    let line2_rev = Line::from_points(l2.1, l2.0).unwrap();

    // 1.
    assert_eq!(expected, line1.intersection(&line2));
    assert_eq!(expected, line2.intersection(&line1));
    assert_eq!(expected.is_some(), line1.intersects(&line2));
    assert_eq!(expected.is_some(), line2.intersects(&line1));

    // 2.
    assert_eq!(expected, line1_rev.intersection(&line2));
    assert_eq!(expected, line2.intersection(&line1_rev));
    assert_eq!(expected.is_some(), line1_rev.intersects(&line2));
    assert_eq!(expected.is_some(), line2.intersects(&line1_rev));

    // 3.
    assert_eq!(expected, line1.intersection(&line2_rev));
    assert_eq!(expected, line2_rev.intersection(&line1));
    assert_eq!(expected.is_some(), line1.intersects(&line2_rev));
    assert_eq!(expected.is_some(), line2_rev.intersects(&line1));

    // 4.
    assert_eq!(expected, line1_rev.intersection(&line2_rev));
    assert_eq!(expected, line2_rev.intersection(&line1_rev));
    assert_eq!(expected.is_some(), line1_rev.intersects(&line2_rev));
    assert_eq!(expected.is_some(), line2_rev.intersects(&line1_rev));
}

#[test]
fn lines_intersect_perpendicular() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(1, -1), &Point::new(1, 1));

    check_lines_intersect(
        line1,
        line2,
        Some(LineIntersection::Point(Point::new(1, 0))),
    );
}

#[test]
fn lines_intersect_touch_perpendicular() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(0, 0), &Point::new(0, 1));

    check_lines_intersect(
        line1,
        line2,
        Some(LineIntersection::Point(Point::new(0, 0))),
    )
}

#[test]
fn lines_intersect_parallel() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(1, 0), &Point::new(3, 0));

    check_lines_intersect(
        line1,
        line2,
        Some(LineIntersection::Line(
            Line::from_points(Point::new(1, 0), Point::new(2, 0)).unwrap(),
        )),
    );
}

#[test]
fn lines_intersect_touch_parallel() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(2, 0), &Point::new(3, 0));

    check_lines_intersect(
        line1,
        line2,
        Some(LineIntersection::Point(Point::new(2, 0))),
    );
}

#[test]
fn lines_intersect_perpendicular_disjoint() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(1, 1), &Point::new(1, 2));

    check_lines_intersect(line1, line2, None);
}

#[test]
fn lines_intersect_parallel_disjoint() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(3, 0), &Point::new(4, 0));

    check_lines_intersect(line1, line2, None);
}

#[test]
fn half_line_intersects_line_true() {
    let point = Point::new(0, 0);
    let dir = Direction::LEFT;

    let line = Line::from_points(Point::new(-1, -1), Point::new(-1, 1)).unwrap();

    let expected_intersection = Some(LineIntersection::Point(Point::new(-1, 0)));
    assert_eq!(
        expected_intersection,
        line.intersection_with_half_line(&point, dir)
    );
    assert!(line.intersects_half_line(&point, dir));
}

#[test]
fn half_line_intersects_line_false_wrong_direction() {
    let point = Point::new(0, 0);
    let dir = Direction::RIGHT;

    let line = Line::from_points(Point::new(-1, -1), Point::new(-1, 1)).unwrap();
    assert_eq!(None, line.intersection_with_half_line(&point, dir));
    assert!(!line.intersects_half_line(&point, dir));
}

#[test]
fn half_line_intersects_line_false_above_half_line() {
    let point = Point::new(0, 0);
    let dir = Direction::LEFT;

    let line = Line::from_points(Point::new(-1, -2), Point::new(-1, -1)).unwrap();
    assert_eq!(None, line.intersection_with_half_line(&point, dir));
    assert!(!line.intersects_half_line(&point, dir));
}

#[test]
fn line_intersections() {
    let points1 = [
        Point::new(0, 0),
        Point::new(50, 0),
        Point::new(50, 50),
        Point::new(0, 50),
    ];

    let points2 = [
        Point::new(25, 25),
        Point::new(75, 25),
        Point::new(75, 75),
        Point::new(25, 75),
        Point::new(25, 25),
    ];

    let lines1 = points1
        .iter()
        .zip(points1.iter().skip(1))
        .map(|(p1, p2)| Line::from_points(p1, p2).unwrap());

    let lines2 = points2
        .iter()
        .zip(points2.iter().skip(1))
        .map(|(p1, p2)| Line::from_points(p1, p2).unwrap());

    let expected = vec![
        LineIntersection::Point(Point::new(50, 25)),
        LineIntersection::Point(Point::new(25, 50)),
    ];

    let res = intersections_line_iters(lines1, lines2).collect::<Vec<_>>();

    assert_eq!(expected, res);
}
