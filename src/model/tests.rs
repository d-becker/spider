use super::*;

mod point {
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

    #[test]
    fn add_points() {
        let a = Point::new(2, -4);
        let b = Point::new(10, -5);

        let exp = Point::new(12, -9);
        let res = a.add(b);

        assert_eq!(exp, res);
    }
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

#[cfg(test)]
mod spider {
    use super::*;

    fn get_spider() -> Spider {
        let null_point = Point::new(0, 0);
        Spider::new(null_point, Direction::NONE)
    }

    fn get_spider_on_line() -> Spider {
        Spider {
            dir: Direction::RIGHT,
            points: vec![Point::origin(), Point::new(1, 0)],
        }
    }

    #[test]
    fn spider_get_pos() {
        let mut spider = get_spider();

        spider.set_dir(Direction::UP);
        assert_eq!(Direction::UP, spider.get_dir());

        spider.set_dir(Direction::DOWN);
        assert_eq!(Direction::DOWN, spider.get_dir());
    }

    #[test]
    fn spider_update_no_motion() {
        let mut spider = get_spider();
        spider.set_dir(Direction::NONE);

        let old_pos = spider.get_pos();
        let old_points = spider.get_points().to_vec();

        spider.update();

        let new_pos = spider.get_pos();
        let new_points = spider.get_points();

        assert_eq!(old_pos, new_pos);
        assert_eq!(old_points, new_points);
    }

    #[test]
    fn spider_simple_motion() {
        let mut spider = get_spider();
        let dir = Direction::UP;
        spider.set_dir(dir);

        let mut old_points = spider.get_points().to_vec();
        let last = old_points.last().map(|&p| p).unwrap_or(Point::origin());
        old_points.push(last.add(dir.to_point()));
        let exp = old_points;

        spider.update();

        let new_points = spider.get_points();

        assert_eq!(exp, new_points);
    }

    #[test]
    fn spider_collinear_motion() {
        let mut spider = get_spider_on_line();

        let dir = spider.get_dir();
        let old_pos = spider.get_pos();
        let old_points_len = spider.get_points().len();

        spider.update();

        let new_pos = spider.get_pos();
        let new_points_len = spider.get_points().len();

        assert_eq!(old_pos.add(dir.to_point()), new_pos);
        assert_eq!(old_points_len, new_points_len);
    }

    #[test]
    fn spider_loop_resolved() {
        let points = vec![
            Point::origin(),
            Point::new(10, 0),
            Point::new(10, 1),
            Point::new(8, 1),
        ];

        let dir = Direction::UP;
        let mut spider = Spider { dir, points };
        spider.update();

        let exp = vec![Point::origin(), Point::new(8, 0)];
        let new_points = spider.get_points();

        assert_eq!(exp, new_points);
    }

    #[test]
    fn spider_loop_and_collinearity_resolved() {
        let points = vec![
            Point::origin(),
            Point::new(10, 0),
            Point::new(10, 2),
            Point::new(8, 2),
            Point::new(8, 1),
        ];

        let dir = Direction::UP;
        let mut spider = Spider { dir, points };
        spider.update();

        let exp = vec![Point::origin(), Point::new(8, 0)];
        let new_points = spider.get_points();

        assert_eq!(exp, new_points);
    }
}
