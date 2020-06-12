use super::*;

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
        let last = old_points.last().copied().unwrap_or(Point::origin());
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
}
