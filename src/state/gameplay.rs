use amethyst::{
  core::transform::Transform,
  prelude::*,
  renderer::{Camera},
  ui::{Anchor, UiTransform},
};

use crate::sprite::storage::SpriteSheetStorage;
use crate::component::player::Player;
use crate::component::def::Side;
use crate::component::score::ScoreText;
use crate::component::rule::Rules;
use super::def::CurrentState;
use super::def::Game;
use super::def::UserAction;

pub struct GameplayState;
impl SimpleState for GameplayState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    world.register::<Player>();

    let storage = SpriteSheetStorage::new(world);
    world.insert(storage);

    initialise_scoreboard(world);
    initialise_players(world);
    initialise_camera(world);
  }
}

fn initialise_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(super::ARENA_WIDTH * 0.5, super::ARENA_HEIGHT * 0.5, 1.0);

  world
      .create_entity()
      .with(Camera::standard_2d(super::ARENA_WIDTH, super::ARENA_HEIGHT))
      .with(transform)
      .build();
}

fn initialise_players(world: &mut World) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the players.
    let y = super::ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(crate::component::PLAYER_WIDTH * 1.0, y, 0.0);
    right_transform.set_translation_xyz(super::ARENA_WIDTH - crate::component::PLAYER_WIDTH * 1.0, y, 0.0);

    let user_player     = Player::new(Side::User);
    let user_player_sprite_render = user_player.get_sprite_render(world).clone();
    
    let opponent_player = Player::new(Side::Opponent);
    let opponent_player_sprite_render = opponent_player.get_sprite_render(world).clone();

    world
    .create_entity()
    .with(user_player_sprite_render)
    .with(user_player)
    .with(right_transform)
    .build()
    ;

    world
    .create_entity()
    .with(opponent_player_sprite_render)
    .with(opponent_player)
    .with(left_transform)
    .build()
    ;

} 

fn initialise_scoreboard(world: &mut World) {
  
  let user_transform = UiTransform::new(
      "opponent".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
      -150., -150., 1., 200., 150.,
  );
  
  let opponent_transform = UiTransform::new(
      "user".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
      150., -150., 1., 200., 150.,
  );

  let rules_transform = UiTransform::new(
    "rule".to_string(), Anchor::BottomMiddle, Anchor::BottomMiddle,
    0., 150., 1., 1000., 50.,
  );
  
  let score_text = ScoreText::new(world, user_transform, opponent_transform);
  world.insert(score_text);

  let rules = Rules::new(world, rules_transform);
  world.insert(rules);
}