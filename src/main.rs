use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::window::{ExitCondition, PrimaryWindow};
use leafwing_input_manager::prelude::*;
use rand::prelude::*;

fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings.backends = Some(Backends::VULKAN);
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.2)))
        .add_plugins((DefaultPlugins
                          .set(RenderPlugin {
                              render_creation: wgpu_settings.into(),
                              ..default()
                          })
                          .set(WindowPlugin {
                              primary_window: Some(Window {
                                  resolution: (800.0, 480.0).into(),
                                  title:"Armies Fighting (It's All They Know)".to_string(),
                                  ..default()
                              }),
                              exit_condition: ExitCondition::OnPrimaryClosed,
                              close_when_requested: true
                          }),
                      BattlePlugin
        ))
        .run();
}

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        //    .add_systems(Update, (drop_object, move_objects, process_falling_things, move_bucket));
    }
}

fn setup(mut commands: Commands) {
    // Mark this camera as our main camera so we can query for it later!
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

//     HELPER FUNCTIONS      \\
fn distance_between(a: &Transform, b: &Transform) -> f32 {
    ((a.translation.x - b.translation.x).powi(2) + (a.translation.y - b.translation.y).powi(2)).sqrt()
}

fn update_mouse_coords(
    mut mouse_coords: ResMut<MouseCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_coords.0 = world_position;
    }
}
/*
//      INPUT PROCESSING      \\
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum BucketAction {
    // This is the list of "things in the game I want to be able to do based on input"
    Left,
    Right
}
fn move_bucket(mut query: Query<(&ActionState<BucketAction>, &mut Speed), With<Bucket>>) {
    let (action_state, mut speed) = query.single_mut();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(BucketAction::Left) {
        speed.horizontal -= 300.0;
    }
    else if action_state.just_released(BucketAction::Left) {
        speed.horizontal += 300.0;
    }
    if action_state.just_pressed(BucketAction::Right) {
        speed.horizontal += 300.0;
    }
    else if action_state.just_released(BucketAction::Right) {
        speed.horizontal -= 300.0;
    }
}
*/
//         RESOURCES         \\
#[derive(Component)]
struct MainCamera;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MouseCoords(Vec2);

//         COMPONENTS        \\
enum CompanyType {
    Infantry,
    Archers,
    Cavalry
}
#[derive(Component)]
struct Company {
    company_type: CompanyType,
    soldiers: u8,
    injured: u8
}
impl Default for Company {
    fn default() -> Self {
        Self {
            company_type: CompanyType::Infantry,
            soldiers: 100,
            injured: 0
        }
    }
}
