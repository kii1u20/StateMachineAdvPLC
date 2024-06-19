mod state_machine_library;
use crate::state_machine_library::StateMachine;
use crate::state_machine_library::StateTransition;
use crate::state_machine_library::Transition;
use std::collections::HashMap;
use std::rc::Rc;
use rand::Rng;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

fn main() {
    let (mut states, mut parent) = define_state_and_parent!();
    define_action!(action1, {
        println!("Selecting drink in slot 1");
    });
    define_action!(handleError1, {
        println!("Unexpected event recieved! Ignoring...");
    });
    create_states!(states, parent,
        Locked => {
            Coin => [Unlocked] => None,
            Push => [Locked] => None,
            TestEvent => [Locked] => None
        } => {None} + [
        Unlocked => {
            Coin => [Unlocked] => None,
            Push => [Pressed1] => None
        } => {None} + [
            Pressed1 => {
                Push => [Locked] => {action1.clone()}
            } => {handleError1.clone()}
        ]
     ]
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
    println!("Passing an event that is not defined...");
    machine.transition("Undefined".to_string());
    println!("Current state: {:?}", machine.current_state);
}
