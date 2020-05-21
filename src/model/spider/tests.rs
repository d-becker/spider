use super::*;

fn get_spider() -> Spider {
    let null_point = Point::new(0, 0);
    Spider::new(null_point, Direction::NONE)
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
fn spider_update_right() {
    for dir in &[
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
        Direction::NONE,
    ] {
        let mut spider = get_spider();
        let old_pos = spider.pos();

        spider.set_dir(*dir);
        spider.update();

        let expected = old_pos.add(dir.to_point());
        let new_pos = spider.pos();
        assert_eq!(expected, new_pos);
    }
}
