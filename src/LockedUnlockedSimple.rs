mod state_machine_library;
use crate::state_machine_library::StateMachine;
use crate::state_machine_library::StateTransition;
use crate::state_machine_library::Transition;
use std::collections::HashMap;
use std::rc::Rc;
use rand::Rng;

fn main() {
    let (mut states, mut parent) = define_state_and_parent!();
    define_action!(action1, {
        println!("Coins Inserted! Unlocking...");
    });

    create_states!(states, parent,
        Locked => {
            Coin => [Unlocked] => {action1.clone()},
            Push => [Locked] => None
        } => {None},
        Unlocked => {
            Coin => [Unlocked] => None,
            Push => [Locked] => None
        } => {None}
    );

    create_state_machine!(states, parent);

    let mut machine = Machine::new("Locked".to_string(), states, parent);

    println!("Current state: {:?}", machine.current_state);
    println!("Executing Coin transition...");
    machine.transition("Coin".to_string());
    println!("Current state: {:?}", machine.current_state);
    println!("Executing Push transition...");
    machine.transition("Push".to_string());
    println!("Current state: {:?}", machine.current_state);
}
