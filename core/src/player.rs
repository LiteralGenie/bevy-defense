use bevy::ecs::{ bundle::Bundle, component::Component, system::Commands };
use serde::{ Serialize };
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue };

#[derive(Component, Serialize)]
pub struct PlayerGold(pub u16);

#[derive(Component, Serialize)]
pub struct PlayerHealth(pub u16);

#[derive(Bundle, Serialize)]
struct PlayerBundle {
    pub gold: PlayerGold,
    pub health: PlayerHealth,
}

pub fn load_players(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        gold: PlayerGold(100),
        health: PlayerHealth(20),
    });
}
