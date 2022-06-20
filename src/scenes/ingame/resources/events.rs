use super::BulletItem;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;

#[derive(Component)]
pub struct Collider;

#[derive(Component, Debug, Reflect)]
pub struct BlockData {
    pub health: u8,
    pub alive: bool,
}

impl Default for BlockData {
    fn default() -> Self {
        Self {
            health: Default::default(),
            alive: true,
        }
    }
}

impl BlockData {
    pub fn deal_damage(&mut self, amount: u8) {
        self.health = self.health.saturating_sub(amount);
        if self.health < 1 {
            self.alive = false;
        }
    }
}

pub struct BulletCollisionEvent {
    pub entity: Entity,
}

///
/// Note: could check for bullet_hits in bullet_move instead of having a sparate system.
/// I believe that the bullet can jump over a Collider, if the speed is high enough:
///  in that case its difficult to catch a missed Collider here,
///   instead
///  its doable during the "move" calculation.
///
pub fn check_or_bullet_collisions(
    mut commands: Commands,
    q_bull: Query<Option<(Entity, &Transform, &BulletItem, &Sprite)>>,
    collider_query: Query<(Entity, &Transform, &Sprite), With<Collider>>,
    mut collision_event: EventWriter<BulletCollisionEvent>,
) {
    for current_bullet in q_bull.iter() {
        if let Some((bull_entity, b_trans, bullet, b_sprite)) = current_bullet {
            for (coll_entity, transform, coll_sprite) in collider_query.iter() {
                let collision = collide(
                    b_trans.translation,
                    b_sprite.custom_size.unwrap(),
                    transform.translation,
                    coll_sprite.custom_size.unwrap(),
                );
                if let Some(collision) = collision {
                    commands.entity(bull_entity).despawn();
                    collision_event.send(BulletCollisionEvent {
                        entity: coll_entity,
                    })
                }
            }
        }
    }
}

pub fn bullet_hits(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BlockData, &mut Sprite), With<Collider>>,
    mut events: EventReader<BulletCollisionEvent>,
) {
    for e in events.iter() {
        if let Ok(mut m) = query.get_mut(e.entity) {
            m.1.deal_damage(5);
            if !m.1.alive {
                commands.entity(m.0).despawn();
            }
        }
    }
}
