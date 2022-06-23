use crate::scenes::ingame::constants::*;

use super::{BulletItem, Loot, BULLET_SIZE, BULLET_SPEED};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;
use bevy_kira_audio::Audio;

#[derive(Component)]
pub struct Collider;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct BlockData {
    pub health: u8,
    pub max_health: u8,
    pub alive: bool,
}

impl Default for BlockData {
    fn default() -> Self {
        Self {
            health: Default::default(),
            max_health: Default::default(),
            alive: true,
        }
    }
}

impl BlockData {
    pub fn new(with_health: u8) -> Self {
        Self {
            health: with_health,
            max_health: with_health,
            alive: true,
        }
    }

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
    q_bull: Query<Option<(Entity, &Transform, &BulletItem, &Sprite)>>,
    collider_query: Query<(Entity, &Transform, &Sprite), With<Collider>>,
    mut collision_event: EventWriter<BulletCollisionEvent>,
) {
    for current_bullet in q_bull.iter() {
        if let Some((bull_entity, b_trans, _bullet, _b_sprite)) = current_bullet {
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
}

pub fn bullet_hits(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BlockData, &Transform), With<Collider>>,
    mut events: EventReader<BulletCollisionEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for e in events.iter() {
        if let Ok((entity, mut block_data, transform)) = query.get_mut(e.entity) {
            block_data.deal_damage(BASIC_BULLET_DAMAGE);

            audio.play(asset_server.load(HIT_AUDIO));

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

pub fn health_based_status(mut query: Query<(&mut Sprite, &BlockData)>) {
    for (mut sprite, block) in query.iter_mut() {
        if block.health < block.max_health {
            let red_amt: f32 = block.health as f32 / block.max_health as f32 * -1.0;
            sprite.color = Color::rgb(1.0 + red_amt.sin(), red_amt.tan(), red_amt.tan());
        }
    }
}
