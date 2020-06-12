use super::field::Field;
use super::point::Direction;
use super::snake::Snake;
use super::spider::Spider;

#[derive(Debug)]
pub struct Game {
    field_: Field,
    spider_: Spider,
    snake_: Snake,
    paused_: bool,
    game_over_: bool,
}

impl Game {
    pub fn new(field: Field, spider: Spider, snake: Snake) -> Game {
        Game {
            field_: field,
            spider_: spider,
            snake_: snake,
            paused_: false,
            game_over_: false,
        }
    }

    pub fn field(&self) -> &Field {
        &self.field_
    }

    pub fn spider(&self) -> &Spider {
        &self.spider_
    }

    pub fn snake(&self) -> &Snake {
        &self.snake_
    }

    pub fn handle_up(&mut self) {
        self.spider_.set_dir(Direction::UP);
    }

    pub fn handle_down(&mut self) {
        self.spider_.set_dir(Direction::DOWN);
    }

    pub fn handle_left(&mut self) {
        self.spider_.set_dir(Direction::LEFT);
    }

    pub fn handle_right(&mut self) {
        self.spider_.set_dir(Direction::RIGHT);
    }

    pub fn handle_stop(&mut self) {
        self.spider_.set_dir(Direction::NONE);
    }

    pub fn handle_pause(&mut self) {
        self.paused_ = !self.paused_;
    }

    pub fn update_state(&mut self) {
        if self.game_over_ || self.paused_ {
            return;
        }
        // Update the snake position.
        self.update_snake();

        // Detect spider starting or ending path.
        self.update_spider();

        // Detect snake eating spider.
        if self.spider().pos() == self.snake().pos() {
            self.handle_spider_eaten();
            return;
        }

        // Detect winning.
        // TODO.
    }

    fn update_snake(&mut self) {
        let dir = self.snake_.next_step(&self.field_, &self.spider_);
        let new_pos = self.snake_.pos().add(dir.to_point());
        self.snake_.set_pos(new_pos);
    }

    fn update_spider(&mut self) {
        let free_polygon = self.field_.free_polygon();
        if !free_polygon.is_inside(self.spider_.pos()) {
            self.spider_.start_path();
        }

        self.spider_.update();

        if !free_polygon.is_inside(self.spider_.pos()) {
            let path = self.spider_.stop_path();
            if let Some(path) = path {
                if let Some((poly1, poly2)) = free_polygon.cut(&path) {
                    let snake_point = self.snake().pos();
                    if poly1.is_inside(snake_point) {
                        self.field_.cut(poly1, poly2);
                    } else {
                        self.field_.cut(poly2, poly1);
                    }
                }
            }
        }
    }

    fn handle_spider_eaten(&mut self) {
        self.game_over_ = true;
        println!("Game over.");
    }
}
