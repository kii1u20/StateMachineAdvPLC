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
    define_action!(action2, {
        println!("Selecting drink in slot 2");
    });
    define_action!(action3, {
        println!("Selecting drink in slot 3");
    });

    create_states!(states, parent,
        Locked => {
            Coin => [Unlocked] => None,
            Push => [Locked] => None,
            TestEvent => [Locked] => None
        } => {None} + [
        Unlocked => {
            Coin => [Unlocked] => None,
            Push => [Pressed1, Pressed2, Pressed3] => None
        } => {None} + [
            Pressed1 => {
                Push => [Locked] => {action1.clone()}
            } => {None},
            Pressed2 => {
                Push => [Locked] => {action2.clone()}
            } => {None},
            Pressed3 => {
                Push => [Locked] => {action3.clone()}
            } => {None}
        ]
     ]
    );

    create_state_machine!(states, parent);

    let mut machine = Machine::new("Locked".to_string(), states, parent);

    println!("Current state: {:?}", machine.current_state);
    println!("Executing Coin transition...");
    machine.transition("Coin".to_string());
    println!("Current state: {:?}", machine.current_state);
    println!("Executing Push transition... Randomly selecting a drink...");
    machine.transition("Push".to_string());
    println!("Current state: {:?}", machine.current_state);
    println!("Executing Push transition...");
    machine.transition("Push".to_string());
    println!("Current state: {:?}", machine.current_state);;
}
