mod audio;
mod menu;

use actions_plugin::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::menu::MenuPlugin;

use bevy::app::AppBuilder;
use bevy::prelude::*;

pub struct MouseTrapPlugin;

impl Plugin for MouseTrapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(game_state::GameState::Loading)
            .add_plugin(assets_plugin::AssetsPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(player_plugin::PlayerPlugin)
            .add_plugin(bevy_devtools::DevToolsPlugin);
    }
}
