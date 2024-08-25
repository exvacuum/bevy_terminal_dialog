use bevy::prelude::*;
use bevy_terminal_display::widgets::components::Widget;

use super::widgets::{DialogBox, DialogBoxWidget, InteractTooltip, InteractTooltipWidget, OptionsBox, OptionsBoxWidget};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        InteractTooltip,
        Widget {
            enabled: false,
            depth: 0,
            widget: Box::new(InteractTooltipWidget),
        },
    ));

    commands.spawn((
        DialogBox,
        Widget {
            enabled: false,
            depth: 0,
            widget: Box::<DialogBoxWidget>::default(),
        },
    ));
    commands.spawn((
        OptionsBox,
        Widget {
            enabled: false,
            depth: 0,
            widget: Box::<OptionsBoxWidget>::default(),
        },
    ));
}
