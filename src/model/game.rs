use super::field::Field;
use super::point::Direction;
use super::snake::Snake;
use super::spider::Spider;

#[derive(Debug)]
pub struct Game {
    field_: Field,
    spider_: Spider,
    snake_: Snake,
}

impl Game {
    pub fn new(field: Field, spider: Spider, snake: Snake) -> Game {
        Game {
            field_: field,
            spider_: spider,
            snake_: snake,
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
        unimplemented!();
    }

    pub fn update_state(&mut self) {
        // Update the snake position.
        // TODO.
        self.snake_.set_pos(
            self.snake_.pos().add(
                self.snake_
                    .next_step(&self.field_, &self.spider_)
                    .to_point(),
            ),
        );

        // Set the position of the spider.
        self.spider_.update();

        // Detect snake eating spider.
        // Detect snake starting or ending path.
        // Detect winning.
    }
}
