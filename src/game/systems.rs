use bevy::prelude::*;
use super::*;


pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Back) {
        if simulation_state.0 == SimulationState::Debug {
            commands.insert_resource(NextState(Some(SimulationState::Raycastig)));
            println!("raycastign toggled");
        } else if simulation_state.0 == SimulationState::Raycastig {
            commands.insert_resource(NextState(Some(SimulationState::Debug)));
            println!("Debug toggled");
        }
    }
}
