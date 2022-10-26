use serde::Deserialize;
use termgame::{GameColor, GameStyle, StyledCharacter};

#[derive(Clone, Debug, Deserialize)]
pub enum Block {
    Barrier,
    Cinderblock,
    Flowerbush,
    Grass,
    Object(char),
    Rock,
    Sand,
    Sign(String),
    Water,
}

impl Block {
    pub fn blocks_movement(&self) -> bool {
        match self {
            Block::Barrier => true,
            _ => false,
        }
    }

    pub fn damage(&self) -> i32 {
        match self {
            Block::Water => 1,
            _ => 0,
        }
    }

    pub fn message(&self) -> Option<String> {
        match self {
            Block::Sign(message) => Some(message.clone()),
            _ => None,
        }
    }
}

impl Into<StyledCharacter> for Block {
    fn into(self) -> StyledCharacter {
        match self {
            Block::Barrier => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::White))),
            Block::Cinderblock => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::LightRed))),
            Block::Flowerbush => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Magenta))),
            Block::Grass => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Green))),
            Block::Object(ch) => StyledCharacter::new(ch),
            Block::Rock => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Gray))),
            Block::Sand => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Yellow))),
            Block::Sign(_) => StyledCharacter::new('💬'),
            Block::Water => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Blue))),
        }
    }
}
