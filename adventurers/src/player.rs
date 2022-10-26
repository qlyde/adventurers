use adventurers_quest::{Event, Quest, QuestStatus};
use blocks::Block;
use termgame::{Game, Message, StyledCharacter, ViewportLocation};

use crate::config::{PLAYER_HEALTH, VP_BUFFER, VP_SIZE};
use crate::movement::{CardinalDirection, Coordinate};
use crate::Map;

/// The player in the game
pub struct Player {
    /// The icon to render the player as
    icon: char,
    /// The current position of the player on the map
    position: Coordinate,
    /// The current health of the player
    health: i32,
    /// Whether the player has won the game or not
    pub won: bool,
}

impl Player {
    /// Move the player and update the game screen and quest
    pub fn do_move(
        &mut self,
        game: &mut Game,
        map: &Map,
        quest: &mut Box<dyn Quest<Event>>,
        card_dir: CardinalDirection,
    ) {
        if self.health == 0 {
            game.end_game();
        }

        let old_pos = self.position;
        let new_pos = self.position + card_dir;
        let destination_block = map.get(&new_pos.into());

        if let Some(destination_block) = destination_block {
            // check if destination block can be walked on
            if destination_block.blocks_movement() {
                return;
            }

            // check if destination block should prompt a message
            if let Some(message) = destination_block.message() {
                game.set_message(Some(Message::new(message)));
            } else {
                game.set_message(None)
            }

            // check if destination block hurts the player
            if destination_block.damage() != 0 {
                self.health -= destination_block.damage();
            } else {
                // player restores health
                self.health = PLAYER_HEALTH;
            }

            // check if the player has just died
            if self.health == 0 {
                game.set_message(Some(Message::new(String::from("You drowned :("))));
            }

            // register event
            if quest.register_event(&Event::on_block(destination_block.clone()))
                == QuestStatus::Complete
            {
                self.won = true;
                game.set_message(Some(
                    Message::new(String::from(
                        "You completed all quests! Press any key to quit",
                    ))
                    .title(String::from("You won!")),
                ));
            }
        }

        self.position = new_pos;
        self.reset_block(game, map, old_pos);
        self.render(game);
    }

    /// Render the player on the game screen
    pub fn render(&self, game: &mut Game) {
        let sc = game.get_screen_char(self.position.x, self.position.y);
        game.set_screen_char(
            self.position.x,
            self.position.y,
            Some(sc.map_or(self.icon.into(), |styled_character| {
                styled_character.character(self.icon)
            })),
        );
        self.move_viewport(game);
    }

    /// Reset a block on the game screen after it has been walked on
    ///
    /// After Object blocks are walked on however, they disappear
    fn reset_block(&self, game: &mut Game, map: &Map, position: Coordinate) {
        let block = map.get(&position.into());
        game.set_screen_char(
            position.x,
            position.y,
            block.map(|b| {
                if let Block::Object(_) = b {
                    let sc: StyledCharacter = b.clone().into();
                    sc.character(' ')
                } else {
                    b.clone().into()
                }
            }),
        );
    }

    /// Move the viewport if the player is close to the edge of the screen
    fn move_viewport(&self, game: &mut Game) {
        let vp = game.get_viewport();
        let mut vp_x = vp.x;
        let mut vp_y = vp.y;

        if self.position.x - vp.x == VP_BUFFER {
            vp_x -= 1;
        } else if vp.x + VP_SIZE.0 - 1 - self.position.x == VP_BUFFER {
            vp_x += 1;
        }

        if self.position.y - vp.y == VP_BUFFER {
            vp_y -= 1;
        } else if vp.y + VP_SIZE.1 - 1 - self.position.y == VP_BUFFER {
            vp_y += 1;
        }

        game.set_viewport(ViewportLocation { x: vp_x, y: vp_y });
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            icon: '♟',
            position: Coordinate::new(2, 2),
            health: PLAYER_HEALTH,
            won: false,
        }
    }
}
