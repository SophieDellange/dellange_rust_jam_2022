use bevy::prelude::*;

use crate::resources::prelude::*;

pub struct Housing {
    bundle: NodeBundle,
}

impl Default for Housing {
    fn default() -> Housing {
        Housing {
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

impl Housing {
    #[must_use]
    pub fn new(width: Val, height: Val) -> Housing {
        let mut housing = Housing::default();
        let housing_size = &mut housing.bundle.style.size;

        housing_size.width = width;
        housing_size.height = height;

        housing
    }

    #[must_use]
    pub fn full() -> Housing {
        Housing::default()
    }

    #[must_use]
    pub fn percent(width: f32, height: f32) -> Housing {
        let mut housing = Housing::default();
        let housing_size = &mut housing.bundle.style.size;

        housing_size.width = Val::Percent(width);
        housing_size.height = Val::Percent(height);

        housing
    }

    pub fn justify_content(&mut self, justify_content: JustifyContent) -> &mut Housing {
        self.bundle.style.justify_content = justify_content;
        self
    }

    pub fn align_items(&mut self, align_items: AlignItems) -> &mut Housing {
        self.bundle.style.align_items = align_items;
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
        parent.spawn_bundle(self.bundle).with_children(children);
    }
}
