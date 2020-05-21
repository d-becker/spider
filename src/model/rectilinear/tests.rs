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
fn collinear_vertical() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(1, 3);
    let line = Line::from_points(p1, p2).unwrap();
    let p3 = Point::new(1, 4);

    assert!(collinear(&line, p3));
}

#[test]
fn collinear_horizontal() {
    let p1 = Point::new(2, 1);
    let p2 = Point::new(3, 1);
    let line = Line::from_points(p1, p2).unwrap();

    let p3 = Point::new(4, 1);

    assert!(collinear(&line, p3));
}

#[test]
fn collinear_false() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(1, 3);
    let line = Line::from_points(p1, p2).unwrap();

    let p3 = Point::new(2, 4);

    assert!(!collinear(&line, p3));
}

#[test]
fn line_contains_inside() {
    let start = Point::new(2, 1);
    let end = Point::new(4, 1);
    let line = Line::from_points(start, end).unwrap();

    let p = Point::new(3, 1);

    assert!(line_contains(&line, p));
}

#[test]
fn line_contains_on_edge() {
    let start = Point::new(2, 1);
    let end = Point::new(4, 1);
    let line = Line::from_points(start, end).unwrap();

    let p = end;

    assert!(line_contains(&line, p));
}

#[test]
fn line_contains_inside_vertical_reversed() {
    let start = Point::new(1, 4);
    let end = Point::new(1, 2);
    let line = Line::from_points(start, end).unwrap();

    let p = Point::new(1, 3);

    assert!(line_contains(&line, p));
}

#[test]
fn line_contains_inside_horizontal_reversed() {
    let start = Point::new(4, 1);
    let end = Point::new(2, 1);
    let line = Line::from_points(start, end).unwrap();

    let p = Point::new(3, 1);

    assert!(line_contains(&line, p));
}

fn check_lines_intersect(l1: (&Point, &Point), l2: (&Point, &Point), expected_eq: bool) {
    let line1 = Line::from_points(l1.0, l1.1).unwrap();
    let line1_rev = Line::from_points(l1.1, l1.0).unwrap();

    let line2 = Line::from_points(l2.0, l2.1).unwrap();
    let line2_rev = Line::from_points(l2.1, l2.0).unwrap();

    // 1.
    assert_eq!(expected_eq, lines_intersect(&line1, &line2));

    // 2.
    assert_eq!(expected_eq, lines_intersect(&line1_rev, &line2));

    // 3.
    assert_eq!(expected_eq, lines_intersect(&line1, &line2_rev));

    // 4.
    assert_eq!(expected_eq, lines_intersect(&line1_rev, &line2_rev));
}

#[test]
fn lines_intersect_perpendicular() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(1, -1), &Point::new(1, 1));

    check_lines_intersect(line1, line2, true);
}

#[test]
fn lines_intersect_touch_perpendicular() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(0, 0), &Point::new(0, 1));

    check_lines_intersect(line1, line2, true);
}

#[test]
fn lines_intersect_parallel() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(1, 0), &Point::new(3, 0));

    check_lines_intersect(line1, line2, true);
}

#[test]
fn lines_intersect_touch_parallel() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(2, 0), &Point::new(3, 0));

    check_lines_intersect(line1, line2, true);
}

#[test]
fn lines_intersect_perpendicular_disjoint() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(1, 1), &Point::new(1, 2));

    check_lines_intersect(line1, line2, false);
}

#[test]
fn lines_intersect_parallel_disjoint() {
    let line1 = (&Point::new(0, 0), &Point::new(2, 0));
    let line2 = (&Point::new(3, 0), &Point::new(4, 0));

    check_lines_intersect(line1, line2, false);
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
    let mut path = Path::with_points(old_points).unwrap();

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
fn path_loop_resolved() {
    let points = [
        Point::origin(),
        Point::new(10, 0),
        Point::new(10, 1),
        Point::new(8, 1),
    ];

    let mut path = Path::with_points(&points).unwrap();
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

    let mut path = Path::with_points(&points).unwrap();
    let new_point = Point::new(8, -1);
    path.add(new_point).unwrap();

    let exp = [Point::origin(), Point::new(8, 0), Point::new(8, -1)];
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
    let path = Path::with_points(&points).unwrap();

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
    let path = Path::with_points(&points).unwrap();

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
    let path = Path::with_points(&points).unwrap();

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
    let path = Path::with_points(&[Point::new(0, 0), Point::new(1, 0)]).unwrap();
    let poly = Polygon::with_path(path);

    assert_eq!(Some(PolygonError::NotEnoughVertices), poly.err());
}

#[test]
fn polygon_three_vertices_error() {
    let path = Path::with_points(&[Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)]).unwrap();
    let poly = Polygon::with_path(path);

    assert_eq!(Some(PolygonError::NotEnoughVertices), poly.err());
}

#[test]
fn polygon_non_rectilinear_error() {
    let path = Path::with_points(&[
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(8, 10),
    ])
    .unwrap();
    let poly = Polygon::with_path(path);

    assert_eq!(Some(PolygonError::NonRectilinear), poly.err());
}

#[test]
fn polygon_self_intersecting_end_error() {
    let path = Path::with_points(&[
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(-2, 10),
        Point::new(-2, 20),
        Point::new(10, 20),
        Point::new(10, 30),
        Point::new(0, 30),
    ])
    .unwrap();
    let poly = Polygon::with_path(path);

    assert_eq!(Some(PolygonError::SelfIntersecting), poly.err());
}

#[test]
fn polygon_ok() {
    let path = Path::with_points(&[
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ])
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
    let path = Path::with_points(&points).unwrap();
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
    let path = Path::with_points(&points).unwrap();
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
    let path = Path::with_points(&points).unwrap();
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
    let path = Path::with_points(&points).unwrap();
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
    let path = Path::with_points(&points).unwrap();
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

    let path1 = Path::with_points(&points1).unwrap();
    let path2 = Path::with_points(&points2).unwrap();
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
    let poly_path = Path::with_points(&points).unwrap();
    let poly_orig = Polygon::with_path(poly_path.clone()).unwrap();

    for i in 0..points.len() {
        points.rotate_left(i);
        let path = Path::with_points(&points).unwrap();
        let poly = Polygon::with_path(path.clone()).unwrap();
        assert_eq!(poly_orig, poly);

        let reversed_points = points.iter().rev().map(|&p| p).collect::<Vec<Point>>();
        let path_rev = Path::with_points(&reversed_points[..]).unwrap();
        let poly_rev = Polygon::with_path(path_rev.clone()).unwrap();
        assert_eq!(poly_orig, poly_rev);
    }
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
    let poly_path = Path::with_points(&points).unwrap();
    let poly_orig = Polygon::with_path(poly_path.clone()).unwrap();

    for i in 0..points.len() {
        let collected = poly_orig.vertex_iter_from_ind(i).collect::<Vec<_>>();
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
    let poly_path = Path::with_points(&points).unwrap();
    let poly_orig = Polygon::with_path(poly_path.clone()).unwrap();

    for i in 0..points.len() {
        let expected = points.iter().rev().map(|&p| p).collect::<Vec<_>>();
        let collected = poly_orig.vertex_iter_from_ind(i).rev().collect::<Vec<_>>();
        assert_eq!(expected, collected, "{:?}", i);

        points.rotate_left(1);
    }
}

fn polygon_test_cut_path_both_directions(
    orig_points: &[Point],
    expected1_points: &[Point],
    expected2_points: &[Point],
    cut_path: &Path,
) {
    let orig_path = Path::with_points(orig_points).unwrap();
    let orig = Polygon::with_path(orig_path).unwrap();

    let expected1_path = Path::with_points(&expected1_points).unwrap();
    let expected1 = Polygon::with_path(expected1_path).unwrap();

    let expected2_path = Path::with_points(&expected2_points).unwrap();
    let expected2 = Polygon::with_path(expected2_path).unwrap();

    {
        let (poly1, poly2) = orig.cut(cut_path).unwrap();
        let res = (&poly1, &poly2);
        assert!(res == (&expected1, &expected2) || res == (&expected2, &expected2));
    }

    {
        let cut_path_reversed = Path::with_points(
            &cut_path
                .points()
                .iter()
                .rev()
                .map(|&p| p)
                .collect::<Vec<_>>(),
        )
        .unwrap();
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

    let cutting_path = Path::with_points(&[Point::new(15, 0), Point::new(15, 10)]).unwrap();

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

    let cutting_path = Path::with_points(&[Point::new(10, 0), Point::new(10, 10)]).unwrap();

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

    let cutting_path = Path::with_points(&[Point::new(10, 10), Point::new(10, 20)]).unwrap();

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

    let cutting_path = Path::with_points(&[
        Point::new(1, 0),
        Point::new(1, 1),
        Point::new(9, 1),
        Point::new(9, 0),
    ])
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

// TODO: Test
//  - insertion point of both ends of path are the same
//      - the other parts are outside?
//      - both outside and inside?
//  - cut path start AND end is at a vertex
//  - three or more `insertion points'
