use std::error::Error;
use std::time::Duration;
use termgame::{
    run_game, Controller, Game, GameEvent, GameSettings, KeyCode, SimpleEvent, StyledCharacter,
};

struct MyGame {}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {}

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Char(ch)) => {
                game.set_screen_char(1, 1, Some(StyledCharacter::new(ch)))
            }
            _ => {}
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = MyGame {};

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
