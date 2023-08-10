use rusty_engine::prelude::*;

struct GameState {
    high_score: u32,
    score: u32,
    target_index: i32,
    // spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            target_index: 0,
            // spawn_timer: Timer::from_seconds(1.0, false),
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = SOUTH_WEST;
    player.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // handle collisons
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.score += 1;
            println!("Current score: {}", game_state.score);
        }
    }

    // handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;
    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Right, KeyCode::D]) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Left, KeyCode::A]) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    // handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("target{}", game_state.target_index);
            game_state.target_index += 1;
            let target = engine.add_sprite(label.clone(), SpritePreset::RacingCarYellow);
            target.translation = mouse_location.clone();
            target.collision = true;
        }
    }
}