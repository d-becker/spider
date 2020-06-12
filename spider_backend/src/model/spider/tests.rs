use super::*;

fn get_spider() -> Spider {
    let null_point = Point::new(0, 0);
    let upper_left = Point::new(-100, -100);
    let lower_right = Point::new(100, 100);
    Spider::new(null_point, Direction::NONE, upper_left, lower_right)
}

#[test]
fn spider_set_get_dir() {
    let mut spider = get_spider();

    spider.set_dir(Direction::UP);
    assert_eq!(Direction::UP, spider.get_dir());

    spider.set_dir(Direction::DOWN);
    assert_eq!(Direction::DOWN, spider.get_dir());
}

#[test]
fn spider_update_right_no_path() {
    for dir in &[
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
        Direction::NONE,
    ] {
        let mut spider = get_spider();
        let old_pos = *spider.pos();

        spider.set_dir(*dir);
        spider.stop_path();
        spider.update();

        let expected = old_pos.add(dir.to_point());
        let new_pos = spider.pos();
        assert_eq!(&expected, new_pos);
        assert_eq!(None, spider.path());
    }
}

#[test]
fn spider_update_right_with_path() {
    for dir in &[
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
        Direction::NONE,
    ] {
        let mut spider = get_spider();
        let old_pos = *spider.pos();

        spider.set_dir(*dir);
        spider.start_path();
        spider.update();

        let expected = old_pos.add(dir.to_point());
        let new_pos = spider.pos();

        let expected_path = rectilinear::Path::with_points([old_pos, expected].iter()).unwrap();
        assert_eq!(&expected, new_pos);
        assert_eq!(Some(&expected_path), spider.path());

        spider.stop_path();
        assert_eq!(None, spider.path());
    }
}

#[test]
fn spider_update_out_of_bounds() {
    let null_point = Point::new(0, 0);
    let upper_left = Point::new(0, 0);
    let lower_right = Point::new(100, 100);
    let mut spider = Spider::new(null_point, Direction::NONE, upper_left, lower_right);

    spider.set_dir(Direction::LEFT);
    spider.update();
    let new_pos = spider.pos();
    assert_eq!(&null_point, new_pos);
}
