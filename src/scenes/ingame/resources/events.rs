#[allow(clippy::wildcard_imports)]
use crate::scenes::ingame::constants::*;

use super::{
    BulletItem, Enemy, Loot, Player, Score, BULLET_SIZE, BULLET_SPEED, ENEMY_KILLED_POINTS,
};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_kira_audio::{Audio, AudioChannel};

#[derive(Component)]
pub struct Collider;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct BlockData {
    pub health: u8,
    pub max_health: u8,
    pub alive: bool,
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
/// Note: could check for `bullet_hits` in `bullet_move` instead of having a sparate system.
///
pub fn check_or_bullet_collisions(
    mut commands: Commands,
    q_bull: Query<(Entity, &Transform, Option<&Player>), With<BulletItem>>,
    collider_query: Query<
        (Entity, &Transform, &Sprite, Option<&Player>),
        (With<Collider>, Or<(With<Player>, With<Enemy>)>),
    >,
    mut collision_event: EventWriter<BulletCollisionEvent>,
) {
    for (bull_entity, b_trans, b_from_player) in q_bull.iter() {
        let bullet_hitbox_start =
            b_trans.translation - Vec3::new(BULLET_SPEED - BULLET_SIZE.x, 0., 0.);
        let bullet_hitbox_size = Vec2::new(BULLET_SPEED + BULLET_SIZE.x, BULLET_SIZE.y);

        for (coll_entity, transform, coll_sprite, coll_is_player) in collider_query.iter() {
            let collision = collide(
                bullet_hitbox_start,
                bullet_hitbox_size,
                transform.translation,
                coll_sprite.custom_size.unwrap(),
            );

            if collision.is_some() && (b_from_player.is_some() ^ coll_is_player.is_some()) {
                commands.entity(bull_entity).despawn();
                collision_event.send(BulletCollisionEvent {
                    entity: coll_entity,
                });
            }
        }
    }
}

pub fn bullet_hits(
    mut commands: Commands,
    mut q_collided: Query<(Entity, &mut BlockData, &Transform, Option<&Player>), With<Collider>>,
    mut q_score: Query<&mut Score>,
    mut events: EventReader<BulletCollisionEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for e in events.iter() {
        if let Ok((entity, mut block_data, transform, is_player)) = q_collided.get_mut(e.entity) {
            block_data.deal_damage(BASIC_BULLET_DAMAGE);

            audio.play_in_channel(
                asset_server.load(SOUND_HIT_ENEMY),
                &AudioChannel::new(AUDIO_EFFECTS_CHANNEL.to_owned()),
            );

            if !block_data.alive && is_player.is_none() {
                commands.entity(entity).despawn();

                audio.play_in_channel(
                    asset_server.load(SOUND_ENEMY_DEATH),
                    &AudioChannel::new(AUDIO_EFFECTS_CHANNEL.to_owned()),
                );

                Loot::random().spawn(
                    transform.translation.truncate(),
                    &mut commands,
                    &asset_server,
                );

                // Note that this does not update the text; that's done via change detection.
                //
                q_score.single_mut().0 += ENEMY_KILLED_POINTS;
            }
        }
    }
}

pub fn health_based_status(mut query: Query<(&mut Sprite, &BlockData)>) {
    for (mut sprite, block) in query.iter_mut() {
        if block.health < block.max_health {
            let red_amt = f32::from(block.health) / f32::from(block.max_health) * -1.0;
            sprite.color = Color::rgb(1.0 + red_amt.sin(), red_amt.tan(), red_amt.tan());
        }
    }
}

pub fn update_scoreboard(mut q_score_text: Query<(&mut Text, &Score), Changed<Score>>) {
    if let Ok((mut text, Score(score))) = q_score_text.get_single_mut() {
        text.sections[0].value = format!("{}", score);
    }
}
