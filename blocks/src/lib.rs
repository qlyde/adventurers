use std::fmt::Display;

use serde::Deserialize;
use termgame::{GameColor, GameStyle, StyledCharacter};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
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
        matches!(self, Block::Barrier)
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

impl From<Block> for StyledCharacter {
    fn from(block: Block) -> Self {
        match block {
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

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Barrier => write!(f, "Barrier"),
            Block::Cinderblock => write!(f, "Cinderblock"),
            Block::Flowerbush => write!(f, "Flowerbush"),
            Block::Grass => write!(f, "Grass"),
            Block::Object(ch) => write!(f, "'{ch}'"),
            Block::Rock => write!(f, "Rock"),
            Block::Sand => write!(f, "Sand"),
            Block::Sign(msg) => write!(f, "Sign(\"{msg}\")"),
            Block::Water => write!(f, "Water"),
        }
    }
}
