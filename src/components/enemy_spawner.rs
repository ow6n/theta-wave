use amethyst::ecs::prelude::{Component, DenseVecStorage};

use std::{
    collections::{HashMap},
    vec::Vec,
};

use crate::components::{ConsumablePool, Rigidbody};


pub struct EnemyPool {
    pub available_enemies: Vec<String>,
    pub enemies: HashMap<String, Enemy>,
}

pub struct EnemySpawner {
    pub enemy_pool: EnemyPool,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub enemies_spawned: u32,
}

impl Component for EnemySpawner {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct Enemy {
    pub width: f32,
    pub height: f32,
    pub health: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub barrel_damaged: bool,
    pub sprite_index: usize,
    pub fires: bool,
    pub fire_speed: f32,
    pub fire_reset_timer: f32,
    pub blast_speed: f32,
    pub blast_damage: f32,
    pub defense_damage: f32,
    pub max_speed: f32,
    pub current_velocity_x: f32,
    pub current_velocity_y: f32,
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub deceleration_x: f32,
    pub deceleration_y: f32,
    pub knockback_max_speed: f32,
    pub collision_damage: f32,
    pub consumable_pool: ConsumablePool,
    pub drop_chance: f32,
}

impl Rigidbody for Enemy{
    fn current_velocity_x(&self) ->  f32 {
        self.current_velocity_x
    }
    fn current_velocity_y(&self) -> f32 {
        self.current_velocity_y
    }
    fn acceleration_x(&self) -> f32 { self.acceleration_x }
    fn acceleration_y(&self) -> f32 { self.acceleration_y }
    fn deceleration_x(&self) -> f32 { self.deceleration_x }
    fn deceleration_y(&self) -> f32 { self.deceleration_y }
    fn max_speed(&self) -> f32 { self.max_speed }
    fn knockback_max_speed(&self) -> f32 { self.knockback_max_speed }

    fn set_current_velocity_y(&mut self, value: f32) {
        self.current_velocity_y = value;
    }
    fn set_current_velocity_x(&mut self, value: f32) { self.current_velocity_x = value; }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}