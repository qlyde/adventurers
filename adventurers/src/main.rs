mod block;
mod config;
mod movement;
mod player;

use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use std::{env, fs};

use block::Block;
use movement::CardinalDirection;
use player::Player;
use termgame::{run_game, Controller, Game, GameEvent, GameSettings, KeyCode, SimpleEvent};

pub type Map = HashMap<(i32, i32), Block>;

struct MyGame {
    player: Player,
    map: Map,
}

impl MyGame {
    fn parse_map() -> Map {
        let map_file = env::args()
            .nth(1)
            .expect("You failed to provide a map filename");
        let contents = fs::read_to_string(map_file).expect("Failed to read map file to string");
        ron::from_str(&contents).expect("Failed to read map file as RON")
    }

    fn render_map(&mut self, game: &mut Game) {
        self.map.iter().for_each(|((x, y), block)| {
            game.set_screen_char(*x, *y, Some(block.clone().into()));
        });
    }
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        self.render_map(game);
        self.player.render(game);
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Up) => {
                self.player
                    .r#move(game, &self.map, CardinalDirection::North)
            }
            SimpleEvent::Just(KeyCode::Right) => {
                self.player.r#move(game, &self.map, CardinalDirection::East)
            }
            SimpleEvent::Just(KeyCode::Down) => {
                self.player
                    .r#move(game, &self.map, CardinalDirection::South)
            }
            SimpleEvent::Just(KeyCode::Left) => {
                self.player.r#move(game, &self.map, CardinalDirection::West)
            }
            // SimpleEvent::Just(KeyCode::Char(ch)) => {
            //     game.set_screen_char(1, 1, Some(StyledCharacter::new(ch)))
            // }
            _ => {}
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = MyGame {
        player: Player::default(),
        map: MyGame::parse_map(),
    };

    run_game(
        &mut controller,
        GameSettings::new()
            // The below are the defaults, but shown so you can edit them.
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into())),
    )?;

    println!("Game Ended!");

    Ok(())
}
