#![warn(missing_docs)]

//! Bevy plugin providing a yarnspinner-based dialog system for the bevy_terminal_display plugin
//! and dirworld plugin.

use bevy::prelude::*;

mod systems;
pub mod widgets;
pub mod util;
pub mod actor;

/// Plugin which provides dialog functionality
pub struct TerminalDialogPlugin;

impl Plugin for TerminalDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(actor::ActorPlugin)
            .add_systems(Startup, systems::setup);
    }
}
