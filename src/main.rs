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
        println!("Selecting drink in slot 1. Dispensing...");
    });
    define_action!(action2, {
        println!("Selecting drink in slot 2. Dispensing...");
    });
    define_action!(action3, {
        println!("Selecting drink in slot 3. Dispensing...");
    });
    define_action!(handleErrorAction, {
        println!("Unexpected event recieved! Ignoring...");
    });
    define_action!(dispenseCompleteAction, {
        println!("Enjoy your drink!");
    });

    create_states!(states, parent,
        Locked => {
            Coin => [Unlocked] => None,
            Push => [Locked] => None
        } => {handleErrorAction.clone()} + [
        Unlocked => {
            Coin => [Unlocked] => None,
            Push => [Pressed1, Pressed2, Pressed3] => None
        } => {handleErrorAction.clone()} + [
            Pressed1 => {
                Push => [DispensingItem1] => {action1.clone()}
            } => {handleErrorAction.clone()} + [
                DispensingItem1 => {
                    Complete => [Locked] => {dispenseCompleteAction.clone()}
                } => {handleErrorAction.clone()}
            ],
            Pressed2 => {
                Push => [DispensingItem2] => {action2.clone()}
            } => {handleErrorAction.clone()} + [
                DispensingItem2 => {
                    Complete => [Locked] => {dispenseCompleteAction.clone()}
                } => {handleErrorAction.clone()}
            ],
            Pressed3 => {
                Push => [DispensingItem3] => {action3.clone()}
            } => {handleErrorAction.clone()} + [
                DispensingItem3 => {
                    Complete => [Locked] => {dispenseCompleteAction.clone()}
                } => {handleErrorAction.clone()}
            ]
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
    println!("Inserting extra coins... Will remove current selection.");
    machine.transition("Coin".to_string());
    println!("Current state: {:?}", machine.current_state);

    println!("Executing Push transition...");
    machine.transition("Push".to_string());
    println!("Current state: {:?}", machine.current_state);
    println!("Executing Push transition...");
    machine.transition("Push".to_string());
    println!("Current state: {:?}", machine.current_state);
    println!("Dispensing complete. Executing Complete transition...");
    machine.transition("Ops,wrong event".to_string());
    println!("Current state: {:?}", machine.current_state);
    machine.transition("Complete".to_string());
    println!("Current state: {:?}", machine.current_state);
}
