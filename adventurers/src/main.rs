mod config;
mod movement;
mod player;

use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use std::{env, fs};

use adventurers_quest::quests::combinators::{QuestMajority, QuestRepeat, QuestThen};
use adventurers_quest::quests::{WalkQuest, WalkRepeatQuest};
use adventurers_quest::{Event, Quest};
use blocks::Block;
use movement::CardinalDirection;
use player::Player;
use termgame::{
    run_game, Controller, Game, GameEvent, GameSettings, KeyCode, Message, SimpleEvent,
};

/// The game map
pub type Map = HashMap<(i32, i32), Block>;

/// The state of the adventurers game
struct MyGame {
    /// The player of the game
    player: Player,
    /// The map of the game
    map: Map,
    /// The quest being completed
    quest: Box<dyn Quest<Event>>,
}

impl MyGame {
    /// Parse a RON file containing a [`Map`] value
    ///
    /// The file name must be passed in as the first command line argument
    ///
    /// # Panics
    ///
    /// If the first command line argument is not a correctly formatted RON file with a [`Map`] value
    ///
    /// # Returns
    ///
    /// A [`Map`] object
    fn parse_map() -> Map {
        let map_file = env::args()
            .nth(1)
            .expect("You failed to provide a map filename");
        let contents = fs::read_to_string(map_file).expect("Failed to read map file to string");
        ron::from_str(&contents).expect("Failed to read map file as RON")
    }

    /// Parse the second command line argument as a quest number
    ///
    /// Currently only `q1`, `q2` and `q3` are supported
    ///
    /// # Panics
    ///
    /// If the second command line argument is not either `q1`, `q2` or `q3`
    ///
    /// # Returns
    ///
    /// A new [`Quest`]
    fn parse_quest() -> Box<dyn Quest<Event>> {
        let quest_no = env::args()
            .nth(2)
            .expect("You failed to provide a quest (q1, q2 or q3)");

        match &*quest_no {
            "q1" => Box::new(QuestRepeat::new(Box::new(WalkQuest::new(Block::Sand)), 5)),
            "q2" => Box::new(QuestThen::new(
                Box::new(QuestRepeat::new(
                    Box::new(WalkQuest::new(Block::Object('x'))),
                    5,
                )),
                Box::new(QuestRepeat::new(
                    Box::new(WalkQuest::new(Block::Object('y'))),
                    3,
                )),
            )),
            "q3" => Box::new(QuestMajority::new(
                Box::new(QuestThen::new(
                    Box::new(QuestRepeat::new(Box::new(WalkQuest::new(Block::Sand)), 5)),
                    Box::new(WalkQuest::new(Block::Object('x'))),
                )),
                Box::new(QuestThen::new(
                    Box::new(WalkQuest::new(Block::Object('x'))),
                    Box::new(WalkQuest::new(Block::Grass)),
                )),
                Box::new(QuestRepeat::new(
                    Box::new(WalkRepeatQuest::new(Block::Water, 9)),
                    2,
                )),
            )),
            _ => panic!("That quest does not exist!"),
        }
    }

    /// Render the contained map state onto the [`Game`] screen
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
        // end the game if the player has already won
        if self.player.won {
            game.end_game();
        }

        match event.into() {
            // move up
            SimpleEvent::Just(KeyCode::Up) => self.player.do_move(
                game,
                &mut self.map,
                &mut self.quest,
                CardinalDirection::North,
            ),

            // move right
            SimpleEvent::Just(KeyCode::Right) => self.player.do_move(
                game,
                &mut self.map,
                &mut self.quest,
                CardinalDirection::East,
            ),

            // move down
            SimpleEvent::Just(KeyCode::Down) => self.player.do_move(
                game,
                &mut self.map,
                &mut self.quest,
                CardinalDirection::South,
            ),

            // move left
            SimpleEvent::Just(KeyCode::Left) => self.player.do_move(
                game,
                &mut self.map,
                &mut self.quest,
                CardinalDirection::West,
            ),

            // check quest status
            SimpleEvent::Just(KeyCode::Char('q')) => game.set_message(Some(
                Message::new(self.quest.to_string()).title(String::from("Quest")),
            )),

            // reset quest
            SimpleEvent::Just(KeyCode::Char('r')) => self.quest.reset(),
            _ => {}
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = MyGame {
        player: Player::default(),
        map: MyGame::parse_map(),
        quest: MyGame::parse_quest(),
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
