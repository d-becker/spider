use super::*;

fn get_spider() -> Spider {
    let null_point = Point::new(0, 0);
    Spider::new(null_point, Direction::NONE)
}

fn get_spider_on_line() -> Spider {
    let mut path = rectilinear::Path::with_start(Point::new(0, 0));
    path.add(Point::new(1, 0)).unwrap();

    Spider {
        dir: Direction::RIGHT,
        path: path,
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

// TODO: Test correct new position depending on direction.
