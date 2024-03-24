use bevy::prelude::*;

pub fn match_spawn(
    id: u16,
    mut commands: &mut Commands,
    id_path: u8,
    tick: u32,
) {
    match id {
        0 => {
            super::basic_unit::spawn(&mut commands, id_path, tick);
        }
        1 => {
            super::tank_unit::spawn(&mut commands, id_path, tick);
        }
        _ => panic!("Invalid unit id {}", id),
    }
}
