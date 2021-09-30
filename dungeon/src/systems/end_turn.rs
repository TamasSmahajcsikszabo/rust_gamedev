#![warn(clippy::pedantic)]

use crate::prelude::*;
#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        TurnState::AwaitingInput => return, // immediate exit with 'return'
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };
    *turn_state = new_state; //de-referencing the variable, i.e. directly writing to the stored resource
}
