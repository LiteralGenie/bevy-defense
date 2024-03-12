use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GamePhase {
    #[default]
    BUILD,
    COMBAT,
}
