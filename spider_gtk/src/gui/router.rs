use std::collections::HashMap;

use super::model::game::Game;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum RouterCommand {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    STOP,
    PAUSE,
}

fn command_to_handler(command: RouterCommand) -> fn(&mut Game) {
    match command {
        RouterCommand::UP => Game::handle_up,
        RouterCommand::DOWN => Game::handle_down,
        RouterCommand::LEFT => Game::handle_left,
        RouterCommand::RIGHT => Game::handle_right,
        RouterCommand::STOP => Game::handle_stop,
        RouterCommand::PAUSE => Game::handle_pause,
    }
}

#[derive(Debug)]
pub struct Router<KeyT>
where
    KeyT: Eq,
    KeyT: std::hash::Hash,
{
    key_to_command: HashMap<KeyT, RouterCommand>,
}

impl<KeyT> Router<KeyT>
where
    KeyT: Eq,
    KeyT: std::hash::Hash,
{
    pub fn new() -> Router<KeyT> {
        Router {
            key_to_command: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: KeyT, command: RouterCommand) -> Option<RouterCommand> {
        self.key_to_command.insert(key, command)
    }

    pub fn route(&self, key: KeyT, game: &mut Game) {
        if let Some(command) = self.key_to_command.get(&key) {
            let handler = command_to_handler(*command);
            handler(game);
        }
    }
}
