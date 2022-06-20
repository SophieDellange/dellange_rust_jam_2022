use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Component)]
pub struct InspectableType;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ReflectedType;
