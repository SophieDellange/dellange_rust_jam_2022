use bevy::prelude::*;

use crate::resources::prelude::*;

pub struct Overlay {
    bundle: NodeBundle,
}

impl Default for Overlay {
    fn default() -> Overlay {
        Overlay {
            bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Colors::TRANSPARENT.into(),
                ..Default::default()
            },
        }
    }
}

impl Overlay {
    #[must_use]
    pub fn new() -> Overlay {
        Overlay::default()
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        children: impl FnOnce(&mut ChildBuilder),
        marker: impl Component,
    ) {
        commands
            .spawn_bundle(self.bundle)
            .with_children(children)
            .insert(marker);
    }
}
