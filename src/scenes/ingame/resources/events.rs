use super::BulletItem;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct BulletCollisionEvent;

pub fn check_or_bullet_collisions(
    mut commands: Commands,
    q_bull: Query<Option<(Entity, &Transform, &BulletItem)>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_event: EventWriter<BulletCollisionEvent>,
) {
    for current_bullet in q_bull.iter() {
        if let Some((bull_entity, b_trans, bullet)) = current_bullet {
            println!("Found bullet");
            for (coll_entity, transform) in collider_query.iter() {
                let collision = collide(
                    b_trans.translation,
                    b_trans.scale.truncate(),
                    transform.translation,
                    transform.scale.truncate(),
                );
                if let Some(collision) = collision {
                    commands.entity(bull_entity).despawn();
                    collision_event.send_default();
                    println!("hit!");
                } else {
                    println!("no collision");
                }
            }
        }
    }
}
