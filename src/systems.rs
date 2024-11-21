use std::time::Duration;

use bevy::prelude::*;
use bevy_terminal_display::widgets::components::Widget;

use super::widgets::{DialogBox, DialogBoxWidget, OptionsBox, OptionsBoxWidget};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        DialogBox,
        Widget {
            enabled: false,
            depth: 0,
            widget: Box::new(DialogBoxWidget::new(
                None,
                vec![],
                Duration::from_millis(25),
            )),
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
