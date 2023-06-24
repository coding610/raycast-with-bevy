use bevy::prelude::*;
use player::systems::*;
use player::raycast::systems::*;
use player::raycast::resources::*;
use tile::wall::systems::*;
use tile::ground::systems::*;

pub mod player;
pub mod tile;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSet {
    Player,
    Raycast
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GroundSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TileSet;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets((
            GroundSet.before(TileSet),
            TileSet.before(PlayerSet::Player),
            TileSet.before(PlayerSet::Raycast),
            PlayerSet::Player.before(PlayerSet::Raycast)
            ))
            //------------------RESOURCES----------------
            .insert_resource(RayVars {ray_rotation_step: 0.5, ray_max_depth: 5.0, fov: 90.0})
            //------------------PLAYER----------------
            .add_startup_system(spawn_player)
            .add_systems((
                    player_movement,
                    player_collision
            ).in_set(PlayerSet::Player))
            .add_systems((
                calculate_rays,
                shorten_rays,
                draw_rays,
                flush_rays
            ).chain().in_set(PlayerSet::Raycast))
            .add_system(change_ray_vars.in_set(PlayerSet::Raycast))
            //------------------WALL----------------
            .add_startup_system(spawn_ground.in_set(GroundSet))
            .add_startup_system(spawn_wall.in_set(TileSet));
    }
}
