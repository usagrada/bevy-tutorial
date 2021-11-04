use super::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    // add things to your app here
    app
      .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup.system()))
      .add_system_set(
        // SystemSet::on_update(AppState::MainMenu).with_system(my_simple_system.system()),
        SystemSet::on_update(AppState::MainMenu).with_system(enter_game.system()),
      )
      .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(exit.system()));
  }
}

#[derive(Component)]
pub struct ColorText;

fn exit(mut commands: Commands, query: Query<Entity, With<ColorText>>) {
  for entity in query.iter() {
    println!("exit {:?}", entity);
    commands.entity(entity).despawn(); //.remove::<ColorText>();
  }
}

fn enter_game(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
  // LWin === Mac Left Command, RWin === Mac Right Command
  if keys.pressed(KeyCode::LWin) || keys.pressed(KeyCode::RWin) {
    // exit
    if keys.pressed(KeyCode::Q) {
      println!("game end");
      std::process::exit(0x0100);
    }
  }
  // Game Start
  if keys.pressed(KeyCode::Return) {
    app_state.set(AppState::InGame).unwrap();
    println!("ok");
  }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn_bundle(TextBundle {
      text: Text::with_section(
        "main menu",
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
    .insert(ColorText);
}
