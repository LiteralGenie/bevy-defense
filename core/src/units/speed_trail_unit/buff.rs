use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::{
    scenario::Scenario,
    units::components::{UnitPosition, UnitStatus, UnitStatusTypes},
};

struct Buff {
    duration: u16,
    multiplier: f64,
}

type BuffsByUnit = HashMap<u64, Buff>;

type PathBuffs = Vec<BuffsByUnit>;

#[derive(Resource)]
pub struct BuffsByPath(HashMap<u8, PathBuffs>);

#[derive(Component)]
pub struct SpeedBuff(pub f64);

pub fn init_buff_map(
    mut commands: Commands,
    scenario: Res<Scenario>,
) {
    let mut map = BuffsByPath(HashMap::new());

    for (id, path) in scenario.paths.iter() {
        let mut path_buffs = Vec::new();

        for _ in path.points.iter() {
            path_buffs.push(HashMap::new());
        }

        map.0.insert(*id, path_buffs);
    }

    commands.insert_resource(map);
}

pub fn spawn_speed_buff(
    mut buffs: ResMut<BuffsByPath>,
    units: Query<
        (Entity, &UnitPosition, &UnitStatus),
        With<super::Marker>,
    >,
) {
    for (entity, pos, status) in units.iter() {
        if !matches!(status.0, UnitStatusTypes::ALIVE) {
            continue;
        }

        let path = buffs.0.get_mut(&pos.id_path).unwrap();
        let bin = path.get_mut(pos.dist as usize).unwrap();
        bin.insert(
            entity.to_bits(),
            Buff {
                duration: super::BUFF_DURATION,
                multiplier: super::BUFF_MULTIPLIER,
            },
        );
    }
}

pub fn update_speed_buff_duration(mut buffs: ResMut<BuffsByPath>) {
    for path in buffs.0.values_mut() {
        for bin in path.iter_mut() {
            let mut to_delete = HashSet::new();

            for (id_unit, buff) in bin.iter_mut() {
                buff.duration -= 1;

                if buff.duration == 0 {
                    to_delete.insert(*id_unit);
                }
            }

            for id in to_delete {
                bin.remove(&id);
            }
        }
    }
}

pub fn apply_speed_buff(
    buffs: Res<BuffsByPath>,
    units: Query<(Entity, &UnitPosition)>,
    mut commands: Commands,
) {
    for (entity, pos) in units.iter() {
        let path = buffs.0.get(&pos.id_path).unwrap();
        let bin = path.get(pos.dist as usize).unwrap();
        let buff = bin.values().max_by(|left, right| {
            left.multiplier.partial_cmp(&right.multiplier).unwrap()
        });

        if let Some(buff) = buff {
            commands
                .entity(entity)
                .insert(SpeedBuff(buff.multiplier));
        } else {
            commands.entity(entity).remove::<SpeedBuff>();
        }
    }
}
