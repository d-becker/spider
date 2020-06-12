use super::*;

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

    let expected = {
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
    let mut path = Path::with_points(old_points.iter()).unwrap();

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
fn path_collinear_motion_backwards() {
    let old_points = &[Point::new(0, 0), Point::new(2, 0)];
    let mut path = Path::with_points(old_points.iter()).unwrap();

    let new_point = Point::new(1, 0);

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
fn path_loop_resolved() {
    let points = [
        Point::origin(),
        Point::new(10, 0),
        Point::new(10, 1),
        Point::new(8, 1),
    ];

    let mut path = Path::with_points(points.iter()).unwrap();
    let new_point = Point::new(8, 0);
    path.add(new_point).unwrap();

    let exp = [Point::origin(), Point::new(8, 0)];
    let new_points = path.points();

    assert_eq!(exp, new_points);
}

#[test]
fn path_loop_resolved_when_crossing_without_stopping() {
    let points = [
        Point::origin(),
        Point::new(10, 0),
        Point::new(10, 1),
        Point::new(8, 1),
    ];

    let mut path = Path::with_points(points.iter()).unwrap();
    let new_point = Point::new(8, -1);
    path.add(new_point).unwrap();

    let exp = [Point::origin(), Point::new(8, 0), Point::new(8, -1)];
    let new_points = path.points();

    assert_eq!(exp, new_points);
}

#[test]
fn path_loop_resolved_through_collinear_segment() {
    let points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(8, 10),
        Point::new(8, 20),
        Point::new(10, 20),
        Point::new(10, 20),
    ];

    let mut path = Path::with_points(points.iter()).unwrap();
    let new_point = Point::new(10, -1);
    path.add(new_point).unwrap();

    let exp = [Point::origin(), Point::new(10, 0), Point::new(10, -1)];
    let new_points = path.points();

    assert_eq!(exp, new_points);
}

#[test]
fn path_loop_resolved_into_collinear_segment() {
    let points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(8, 10),
        Point::new(8, 20),
        Point::new(10, 20),
        Point::new(10, 20),
    ];

    let mut path = Path::with_points(points.iter()).unwrap();
    let new_point = Point::new(10, 5);
    path.add(new_point).unwrap();

    let exp = [Point::origin(), Point::new(10, 0), Point::new(10, 5)];
    let new_points = path.points();

    assert_eq!(exp, new_points);
}

#[test]
fn path_insertion_point_found() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let path = Path::with_points(points.iter()).unwrap();

    let point = Point::new(10, 15);

    let expected = Some(4);
    let res = path.insertion_point(&point);

    assert_eq!(expected, res);
}

#[test]
fn path_no_insertion_point_collinear() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let path = Path::with_points(points.iter()).unwrap();

    let point = Point::new(-10, 0);

    let expected = None;
    let res = path.insertion_point(&point);

    assert_eq!(expected, res);
}

#[test]
fn path_no_insertion_point_not_collinear() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let path = Path::with_points(points.iter()).unwrap();

    let point = Point::new(10, 1);

    let expected = None;
    let res = path.insertion_point(&point);

    assert_eq!(expected, res);
}

