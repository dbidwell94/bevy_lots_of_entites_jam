mod factory_state;
mod game_state;
mod styles;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            factory_state::FactoryStateUIPlugin,
            game_state::GameStateUIPlugin,
        ));
    }
}
