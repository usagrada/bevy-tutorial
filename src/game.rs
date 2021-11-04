use super::data;
use super::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    // add things to your app here
    app
      .init_resource::<GameState>()
      .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup.system()))
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .with_system(my_simple_system.system())
          .with_system(text_update_system.system()),
      )
      .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(exit.system()));
  }
}

#[derive(Component)]
pub struct GameText;

#[derive(Default)]
pub struct GameState {
  value: usize,
}

fn exit(mut commands: Commands, query: Query<Entity, With<GameText>>) {
  for entity in query.iter() {
    println!("exit {:?}", entity);
    commands.entity(entity).despawn();
  }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<GameState>) {
  state.value = 0;
  commands
    .spawn_bundle(TextBundle {
      text: Text::with_section(
        "game",
        TextStyle {
          font: asset_server.load(FONT),
          font_size: 30.0,
          color: Color::WHITE,
          ..Default::default()
        },
        TextAlignment::default(),
      ),
      ..Default::default()
    })
    .insert(GameText);
}

fn my_simple_system(
  keys: Res<Input<KeyCode>>,
  btns: Res<Input<MouseButton>>,
  mut state: ResMut<GameState>,
  mut app_state: ResMut<State<AppState>>,
) {
  // Keyboard input
  if keys.just_pressed(KeyCode::Escape) {
    // space is being held down
    app_state.set(AppState::MainMenu).unwrap();
  }
  if keys.just_pressed(KeyCode::Space) {
    // space is being held down
    state.value += 1;
    println!("press space");
  }
  if keys.just_pressed(KeyCode::Return) {
    // space is being held down
    state.value += 1;
    println!("press return");
  }

  // LWin === Mac Left Command, RWin === Mac Right Command
  if keys.pressed(KeyCode::LWin) || keys.pressed(KeyCode::RWin)  {
    // exit
    if keys.pressed(KeyCode::Q) {
      std::process::exit(0x0100);
    }
  }

  // Mouse buttons
  if btns.just_pressed(MouseButton::Left) {
    // a left click just happened
  }
}

pub fn text_update_system(
  // commands: Commands,
  mut query: Query<&mut Text, With<GameText>>,
  state: Res<GameState>,
) {
  for mut text in query.iter_mut() {
    text.sections[0].value = data::TEXT[state.value % data::LEN].to_owned();
  }
}
