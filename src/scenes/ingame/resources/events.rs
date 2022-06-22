use super::{BulletItem, Loot, Player, BULLET_SIZE, BULLET_SPEED};
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
///
pub fn check_or_bullet_collisions(
    mut commands: Commands,
    q_bull: Query<(Entity, &Transform, &BulletItem, &Sprite), With<Player>>,
    collider_query: Query<(Entity, &Transform, &Sprite), With<Collider>>,
    mut collision_event: EventWriter<BulletCollisionEvent>,
) {
    for (bull_entity, b_trans, _bullet, _b_sprite) in q_bull.iter() {
        let bullet_hitbox_start =
            b_trans.translation - Vec3::new(BULLET_SPEED - BULLET_SIZE.x, 0., 0.);
        let bullet_hitbox_size = Vec2::new(BULLET_SPEED + BULLET_SIZE.x, BULLET_SIZE.y);

        for (coll_entity, transform, coll_sprite) in collider_query.iter() {
            let collision = collide(
                bullet_hitbox_start,
                bullet_hitbox_size,
                transform.translation,
                coll_sprite.custom_size.unwrap(),
            );
            if collision.is_some() {
                commands.entity(bull_entity).despawn();
                collision_event.send(BulletCollisionEvent {
                    entity: coll_entity,
                })
            }
        }
    }
}

pub fn bullet_hits(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BlockData, &Transform), With<Collider>>,
    mut events: EventReader<BulletCollisionEvent>,
    asset_server: Res<AssetServer>,
) {
    for e in events.iter() {
        if let Ok((entity, mut block_data, transform)) = query.get_mut(e.entity) {
            block_data.deal_damage(5);
            if !block_data.alive {
                commands.entity(entity).despawn();

                Loot::random().spawn(
                    transform.translation.truncate(),
                    &mut commands,
                    &asset_server,
                );
            }
        }
    }
}
