use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::window::ExitCondition;
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
        //app.add_systems(Startup, setup)
        //    .add_systems(Update, (drop_object, move_objects, process_falling_things, move_bucket));
    }
}


//     COMPONENTS & STUFF     \\
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
