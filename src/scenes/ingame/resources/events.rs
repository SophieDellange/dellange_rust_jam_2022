use super::{Enemy, Loot, Player, Score, BULLET_SIZE, BULLET_SPEED, ENEMY_KILLED_POINTS};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;

#[derive(Component)]
pub struct Collider;

#[derive(Component, Debug, Reflect)]
pub struct BlockData {
    pub health: u8,
    pub alive: bool,
}

impl BlockData {
    pub fn new(health: u8) -> Self {
        Self {
            health,
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
    q_bull: Query<(Entity, &Transform, Option<&Player>), Or<(With<Player>, With<Enemy>)>>,
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
                })
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
) {
    for e in events.iter() {
        if let Ok((entity, mut block_data, transform, is_player)) = q_collided.get_mut(e.entity) {
            block_data.deal_damage(5);
            if !block_data.alive {
                commands.entity(entity).despawn();

                if is_player.is_none() {
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
}

pub fn update_scoreboard(mut q_score_text: Query<(&mut Text, &Score), Changed<Score>>) {
    if let Ok((mut text, Score(score))) = q_score_text.get_single_mut() {
        text.sections[0].value = format!("{}", score);
    }
}
