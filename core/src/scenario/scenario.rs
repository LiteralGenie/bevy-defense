use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Scenario {
    pub paths: HashMap<u8, super::Path>,
    pub waves: Vec<Wave>,
}

pub struct Wave {
    pub enemies: Vec<WaveEnemy>,
}

pub struct WaveEnemy {
    pub id_unit: u16,
    pub id_path: u8,
    pub delay: u32,
}
