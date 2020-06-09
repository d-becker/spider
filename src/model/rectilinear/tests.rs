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

#[test]
fn polygon_empty_ok() {
    let path = Path::new();
    let poly = Polygon::with_path(path);
    assert!(poly.is_ok());
}

#[test]
fn polygon_one_vertex_error() {
    let path = Path::with_start(Point::new(0, 0));
    let poly = Polygon::with_path(path);

    assert_eq!(Some(PolygonError::NotEnoughVertices), poly.err());
}

#[test]
fn polygon_two_vertices_error() {
    let path = Path::with_points([Point::new(0, 0), Point::new(1, 0)].iter()).unwrap();
    let poly = Polygon::with_path(path);

    assert_eq!(Some(PolygonError::NotEnoughVertices), poly.err());
}

#[test]
fn polygon_three_vertices_error() {
    let path =
        Path::with_points([Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)].iter()).unwrap();
    let poly = Polygon::with_path(path);

    assert_eq!(Some(PolygonError::NotEnoughVertices), poly.err());
}

#[test]
fn polygon_non_rectilinear_error() {
    let path = Path::with_points(
        [
            Point::new(0, 0),
            Point::new(10, 0),
            Point::new(10, 10),
            Point::new(8, 10),
        ]
        .iter(),
    )
    .unwrap();
    let poly = Polygon::with_path(path);

    assert_eq!(Some(PolygonError::NonRectilinear), poly.err());
}

#[test]
fn polygon_self_intersecting_end_error() {
    let path = Path::with_points(
        [
            Point::new(0, 0),
            Point::new(10, 0),
            Point::new(10, 10),
            Point::new(-2, 10),
            Point::new(-2, 20),
            Point::new(10, 20),
            Point::new(10, 30),
            Point::new(0, 30),
        ]
        .iter(),
    )
    .unwrap();
    let poly = Polygon::with_path(path);

    assert_eq!(Some(PolygonError::SelfIntersecting), poly.err());
}

#[test]
fn polygon_ok() {
    let path = Path::with_points(
        [
            Point::new(0, 0),
            Point::new(10, 0),
            Point::new(10, 10),
            Point::new(0, 10),
        ]
        .iter(),
    )
    .unwrap();
    let poly = Polygon::with_path(path);

    assert!(poly.is_ok());
}

#[test]
fn empty_polygon_area() {
    let path = Path::new();
    let poly = Polygon::with_path(path).unwrap();

    let exp = 0;
    let res = poly.area();
    assert_eq!(exp, res);
}

#[test]
fn polygon_area_rectangle() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(0, 10),
    ];
    let path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(path).unwrap();

    let exp = 200;
    let res = poly.area();
    assert_eq!(exp, res);
}

#[test]
fn polygon_area_complex() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(path).unwrap();

    let exp = 200 + 100;
    let res = poly.area();
    assert_eq!(exp, res);
}

#[test]
fn polygon_insertion_point_found() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(path).unwrap();

    let point = Point::new(10, 15);

    let expected = Some(4);
    let res = poly.insertion_point(&point);

    assert_eq!(expected, res);
}

#[test]
fn polygon_insertion_point_found_at_end() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(path).unwrap();

    let point = Point::new(0, 10);

    let expected = Some(0);
    let res = poly.insertion_point(&point);

    assert_eq!(expected, res);
}

#[test]
fn polygon_insertion_no_point() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(path).unwrap();

    let point = Point::new(5, 10);

    let expected = None;
    let res = poly.insertion_point(&point);

    assert_eq!(expected, res);
}

#[test]
fn polygon_eq_not_equal() {
    let points1 = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(0, 10),
    ];

    let points2 = [
        Point::new(0, 0),
        Point::new(30, 0),
        Point::new(30, 10),
        Point::new(0, 10),
    ];

    let path1 = Path::with_points(points1.iter()).unwrap();
    let path2 = Path::with_points(points2.iter()).unwrap();
    let poly1 = Polygon::with_path(path1.clone()).unwrap();
    let poly2 = Polygon::with_path(path2.clone()).unwrap();

    assert_ne!(poly1, poly2);
}

#[test]
fn polygon_eq_rotated() {
    let mut points = vec![
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly_orig = Polygon::with_path(poly_path.clone()).unwrap();

    for i in 0..points.len() {
        points.rotate_left(i);
        let path = Path::with_points(points.iter()).unwrap();
        let poly = Polygon::with_path(path.clone()).unwrap();
        assert_eq!(poly_orig, poly);

        let reversed_points = points.iter().rev().copied().collect::<Vec<Point>>();
        let path_rev = Path::with_points(reversed_points[..].iter()).unwrap();
        let poly_rev = Polygon::with_path(path_rev.clone()).unwrap();
        assert_eq!(poly_orig, poly_rev);
    }
}

#[test]
fn polygon_eq_extra_path_point() {
    let points1 = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(0, 10),
    ];

    let points2 = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(0, 10),
        Point::new(0, 5),
    ];

    let path1 = Path::with_points(points1.iter()).unwrap();
    let path2 = Path::with_points(points2.iter()).unwrap();
    let poly1 = Polygon::with_path(path1.clone()).unwrap();
    let poly2 = Polygon::with_path(path2.clone()).unwrap();

    assert_eq!(poly1, poly2);
}

#[test]
fn polygon_iterator() {
    let mut points = vec![
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly_orig = Polygon::with_path(poly_path.clone()).unwrap();

    for i in 0..points.len() {
        let collected = poly_orig
            .vertex_iter_from_ind(i)
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(points, collected);

        points.rotate_left(1);
    }
}

#[test]
fn polygon_iterator_rev() {
    let mut points = vec![
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly_orig = Polygon::with_path(poly_path.clone()).unwrap();

    for i in 0..points.len() {
        let expected = points.iter().rev().copied().collect::<Vec<_>>();
        let collected = poly_orig
            .vertex_iter_from_ind(i)
            .rev()
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(expected, collected, "{:?}", i);

        points.rotate_left(1);
    }
}

#[test]
fn polygon_line_iterator() {
    let points = vec![
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path.clone()).unwrap();

    let expected: Vec<_> = {
        let rotated = points
            .iter()
            .chain(iter::once(points.first().unwrap()))
            .skip(1);
        points
            .iter()
            .zip(rotated)
            .map(|(p1, p2)| Line::from_points(p1, p2).unwrap())
            .collect()
    };

    let res: Vec<_> = poly.line_iter().collect();

    assert_eq!(expected, res);
}

#[test]
fn polygon_inside() {
    let points = vec![
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(20, 20),
        Point::new(20, 30),
        Point::new(0, 30),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path.clone()).unwrap();

    let inside_points = vec![
        Point::new(1, 1),
        Point::new(5, 10),
        Point::new(5, 15),
        Point::new(6, 25),
        Point::new(15, 5),
        Point::new(15, 25),
    ];

    let outside_points = vec![
        Point::new(-1, -1),
        Point::new(15, -5),
        Point::new(-5, 15),
        Point::new(5, 35),
        Point::new(25, 5),
        Point::new(25, 25),
        Point::new(15, 15),
        Point::new(20, 15),
    ];

    for point in &inside_points {
        assert!(poly.is_inside(point), "Should be inside: {:?}.", point);
    }

    for point in &outside_points {
        assert!(!poly.is_inside(point), "Should be outside: {:?}.", point);
    }

    for point in &points {
        assert!(!poly.is_inside(point), "Should be outside: {:?}.", point);
    }
}

#[test]
fn polygon_inside_outside_but_3_intersections_rightwards() {
    let points = [
        Point::new(0, 0),
        Point::new(8, 0),
        Point::new(8, 4),
        Point::new(12, 4),
        Point::new(12, 0),
        Point::new(50, 0),
        Point::new(50, 20),
        Point::new(0, 20),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path.clone()).unwrap();

    let point = Point::new(10, 0);

    assert!(!poly.is_inside(&point));
}

#[test]
fn polygon_inside_outside_but_3_intersections_in_same_direction() {
    let points = [
        Point::new(0, 10),
        Point::new(10, 10),
        Point::new(10, 0),
        Point::new(20, 0),
        Point::new(20, 20),
        Point::new(0, 20),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path.clone()).unwrap();

    let point = Point::new(-2, 10);

    assert!(!poly.is_inside(&point));
}

fn polygon_test_cut_path_both_directions(
    orig_points: &[Point],
    expected1_points: &[Point],
    expected2_points: &[Point],
    cut_path: &Path,
) {
    let orig_path = Path::with_points(orig_points.iter()).unwrap();
    let orig = Polygon::with_path(orig_path).unwrap();

    let expected1_path = Path::with_points(expected1_points.iter()).unwrap();
    let expected1 = Polygon::with_path(expected1_path).unwrap();

    let expected2_path = Path::with_points(expected2_points.iter()).unwrap();
    let expected2 = Polygon::with_path(expected2_path).unwrap();

    {
        let (poly1, poly2) = orig.cut(cut_path).unwrap();
        let res = (&poly1, &poly2);
        assert!(res == (&expected1, &expected2) || res == (&expected2, &expected2));
    }

    {
        let cut_path_reversed = Path::with_points(cut_path.points().iter().rev().copied()).unwrap();
        let (poly1, poly2) = orig.cut(&cut_path_reversed).unwrap();
        let res = (&poly1, &poly2);
        assert!(res == (&expected1, &expected2) || res == (&expected2, &expected2));
    }
}

#[test]
fn polygon_cut_mid_line() {
    let orig_points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];

    let cutting_path = Path::with_points([Point::new(15, 0), Point::new(15, 10)].iter()).unwrap();

    let points1 = [
        Point::new(0, 0),
        Point::new(15, 0),
        Point::new(15, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];

    let points2 = [
        Point::new(15, 0),
        Point::new(15, 10),
        Point::new(20, 10),
        Point::new(20, 0),
    ];

    polygon_test_cut_path_both_directions(&orig_points, &points1, &points2, &cutting_path);
}

#[test]
fn polygon_cut_mid_line_and_vertex() {
    let orig_points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(0, 20),
    ];

    let cutting_path = Path::with_points([Point::new(10, 0), Point::new(10, 10)].iter()).unwrap();

    let points1 = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 20),
        Point::new(0, 20),
    ];

    let points2 = [
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(20, 10),
        Point::new(20, 0),
    ];

    polygon_test_cut_path_both_directions(&orig_points, &points1, &points2, &cutting_path);
}

#[test]
fn polygon_cut_vertex_and_vertex() {
    let orig_points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(20, 10),
        Point::new(20, 20),
        Point::new(10, 20),
        Point::new(10, 30),
        Point::new(0, 30),
    ];

    let cutting_path = Path::with_points([Point::new(10, 10), Point::new(10, 20)].iter()).unwrap();

    let points1 = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 30),
        Point::new(0, 30),
    ];

    let points2 = [
        Point::new(10, 10),
        Point::new(20, 10),
        Point::new(20, 20),
        Point::new(10, 20),
    ];

    polygon_test_cut_path_both_directions(&orig_points, &points1, &points2, &cutting_path);
}

#[test]
fn polygon_cut_same_insertion_point() {
    let orig_points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];

    let cutting_path = Path::with_points(
        [
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(9, 1),
            Point::new(9, 0),
        ]
        .iter(),
    )
    .unwrap();

    let points1 = [
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(1, 1),
        Point::new(9, 1),
        Point::new(9, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];

    let points2 = [
        Point::new(1, 0),
        Point::new(1, 1),
        Point::new(9, 1),
        Point::new(9, 0),
    ];

    polygon_test_cut_path_both_directions(&orig_points, &points1, &points2, &cutting_path);
}

#[test]
fn polygon_cut_path_does_not_start_or_end_on_edge() {
    let points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path).unwrap();

    let cutting_path1 = Path::with_points([Point::new(1, 1), Point::new(9, 1)].iter()).unwrap();

    assert_eq!(None, poly.cut(&cutting_path1));

    let cutting_path2 =
        Path::with_points([Point::new(1, 0), Point::new(1, 1), Point::new(9, 1)].iter()).unwrap();

    assert_eq!(None, poly.cut(&cutting_path2));

    let cutting_path3 =
        Path::with_points([Point::new(1, 1), Point::new(9, 1), Point::new(9, 0)].iter()).unwrap();

    assert_eq!(None, poly.cut(&cutting_path3));
}

#[test]
fn polygon_cut_path_outside() {
    let points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path).unwrap();

    let cutting_path = Path::with_points(
        [
            Point::new(1, 0),
            Point::new(1, -1),
            Point::new(9, -1),
            Point::new(9, 0),
        ]
        .iter(),
    )
    .unwrap();

    assert_eq!(None, poly.cut(&cutting_path));
}

#[test]
fn polygon_cut_path_partly_outside() {
    let points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path).unwrap();

    let cutting_path = Path::with_points(
        [
            Point::new(1, 0),
            Point::new(1, -1),
            Point::new(2, -1),
            Point::new(2, 1),
            Point::new(4, 1),
            Point::new(4, -1),
            Point::new(9, -1),
            Point::new(9, 0),
        ]
        .iter(),
    )
    .unwrap();

    assert_eq!(None, poly.cut(&cutting_path));
}

#[test]
fn polygon_cut_path_line_outside_but_no_points() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(20, 20),
        Point::new(20, 30),
        Point::new(0, 30),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path).unwrap();

    let cutting_path = Path::with_points([Point::new(20, 10), Point::new(20, 20)].iter()).unwrap();

    assert_eq!(None, poly.cut(&cutting_path));
}

#[test]
fn polygon_cut_path_line_outside_but_no_points_multiple_lines() {
    let points = [
        Point::new(0, 0),
        Point::new(20, 0),
        Point::new(20, 10),
        Point::new(10, 10),
        Point::new(10, 20),
        Point::new(20, 20),
        Point::new(20, 30),
        Point::new(0, 30),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path).unwrap();

    let cutting_path = Path::with_points(
        [
            Point::new(20, 5),
            Point::new(15, 5),
            Point::new(15, 25),
            Point::new(20, 25),
        ]
        .iter(),
    )
    .unwrap();

    assert_eq!(None, poly.cut(&cutting_path));
}

#[test]
fn polygon_cut_path_multiple_edge_touches() {
    let points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path).unwrap();

    let cutting_path = Path::with_points(
        [
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(2, 0),
            Point::new(4, 0),
            Point::new(4, 1),
            Point::new(9, 1),
            Point::new(9, 0),
        ]
        .iter(),
    )
    .unwrap();

    assert_eq!(None, poly.cut(&cutting_path));
}

#[test]
fn polygon_cut_path_2_vertex_path_on_edge() {
    let points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path).unwrap();

    let cutting_path = Path::with_points([Point::new(1, 0), Point::new(9, 0)].iter()).unwrap();

    assert_eq!(None, poly.cut(&cutting_path));
}

#[test]
fn polygon_cut_path_1_vertex_path() {
    let points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path).unwrap();

    let cutting_path = Path::with_points([Point::new(9, 0)].iter()).unwrap();

    assert_eq!(None, poly.cut(&cutting_path));
}

#[test]
fn polygon_cut_path_0_vertex_path() {
    let points = [
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];
    let poly_path = Path::with_points(points.iter()).unwrap();
    let poly = Polygon::with_path(poly_path).unwrap();

    let cutting_path = Path::with_points([].iter()).unwrap();

    assert_eq!(None, poly.cut(&cutting_path));
}
