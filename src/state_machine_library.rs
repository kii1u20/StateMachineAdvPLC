use std::collections::HashMap;
use std::rc::Rc;

pub trait StateMachine {
    type State;
    type Event;
    type Action;
    type HandleError;

    fn new(
        initial_state: Self::State,
        transitions: Vec<
            Box<
                dyn Transition<
                    State = Self::State,
                    Event = Self::Event,
                    Action = Self::Action,
                    HandleError = Self::HandleError,
                >,
            >,
        >,
        parent: HashMap<Self::State, Self::State>,
    ) -> Self;
    fn transition(&mut self, event: Self::Event);
}

pub trait Transition {
    type State;
    type Event;
    type Action: Fn() -> ();
    type HandleError: Fn() -> ();

    fn new(
        begin_state: Self::State,
        end_state: Self::State,
        event: Self::Event,
        action: Option<Rc<Self::Action>>,
        handle_error: Option<Rc<Self::HandleError>>,
    ) -> Self
    where
        Self: Sized;
    fn begin_state(&self) -> &Self::State;
    fn end_state(&self) -> &Self::State;
    fn event(&self) -> &Self::Event;
    fn action(&self) -> Option<&Self::Action>;
    fn handle_error(&self) -> Option<&Self::HandleError>;
}

pub struct StateTransition {
    begin_state: String,
    end_state: String,
    event: String,
    action: Option<Rc<Box<dyn Fn() -> ()>>>,
    handle_error: Option<Rc<Box<dyn Fn() -> ()>>>, 
}

impl Transition for StateTransition {
    type State = String;
    type Event = String;
    type Action = Box<dyn Fn() -> ()>;
    type HandleError = Box<dyn Fn() -> ()>;

    fn new(
        begin_state: Self::State,
        end_state: Self::State,
        event: Self::Event,
        action: Option<Rc<Self::Action>>,
        handle_error: Option<Rc<Self::HandleError>>, 
    ) -> Self {
        StateTransition {
            begin_state,
            end_state,
            event,
            action,
            handle_error,
        }
    }

    fn begin_state(&self) -> &Self::State {
        &self.begin_state
    }

    fn end_state(&self) -> &Self::State {
        &self.end_state
    }

    fn event(&self) -> &Self::Event {
        &self.event
    }

    fn action(&self) -> Option<&Self::Action> {
        self.action.as_ref().map(|action| action.as_ref())
    }

    fn handle_error(&self) -> Option<&Self::HandleError> {
        self.handle_error
            .as_ref()
            .map(|handle_error| handle_error.as_ref())
    }
}

#[macro_export]
macro_rules! extract_state_name {
    ( $( $state:ident => { $( $event:ident => [$( $next_state:ident ),*] => $action:expr ),*  } => { $state_handle_error:expr } $(+ [ $( $child_state:tt )* ])? ),* $(,)? ) => {
        vec![
            $(
                stringify!($state).to_string()
            ),*
        ]
    };
}

#[macro_export]
macro_rules! create_states {
    ( $states:expr, $parent:expr, $( $state:ident => { $( $event:ident => [$( $next_state:ident ),*] => $action:expr ),*  } => { $state_handle_error:expr } $(+ [ $( $child_state:tt )* ])? ),* $(,)? ) => {
        $(
            let mut $state: Vec<String> = vec![];

            $(
                create_states!($states, $parent, $($child_state)*);
                $state.extend(extract_state_name!($($child_state)*));
                for child_state in extract_state_name!($($child_state)*).iter() {
                    $parent.insert(child_state.to_string(), stringify!($state).to_string());
                }
            )?

            $(
                for next_state in vec![$(stringify!($next_state).to_string()),*].iter() {
                    $states.push(Box::new(StateTransition::new(
                        stringify!($state).to_string(),
                        next_state.clone(),
                        stringify!($event).to_string(),
                        $action,
                        $state_handle_error
                    )));
                }
            )*
        )*
    };
}

#[macro_export]
macro_rules! create_state_machine {
    ($states:expr, $parent:expr) => {
        struct Machine {
            parent: HashMap<String, String>,
            current_state: String,
            states: Vec<
                Box<
                    dyn Transition<
                        State = String,
                        Event = String,
                        Action = Box<dyn Fn() -> ()>,
                        HandleError = Box<dyn Fn() -> ()>,
                    >,
                >,
            >
        }

        impl StateMachine for Machine {
            type State = String;
            type Event = String;
            type Action = Box<dyn Fn() -> ()>;
            type HandleError = Box<dyn Fn() -> ()>;

            fn new(
                initial_state: String,
                states: Vec<
                    Box<
                        dyn Transition<
                            State = String,
                            Event = String,
                            Action = Box<dyn Fn() -> ()>,
                            HandleError = Box<dyn Fn() -> ()>,
                        >,
                    >,
                >,
                parent: HashMap<String, String>,
            ) -> Self {
                Machine {
                    states,
                    parent,
                    current_state: initial_state,
                }
            }

            fn transition(&mut self, event: Self::Event) {
                let mut current_state = self.current_state.clone();
                let mut handle_error: Option<&Self::HandleError> = None;
                loop {
                    let mut possible_states: Vec<&Box<dyn Transition<State = String, Event = String, Action = Box<dyn Fn() -> ()>, HandleError = Box<dyn Fn() -> ()>>>> = vec![];
                    for state_transition in &self.states {
                        if *state_transition.begin_state() == current_state && !handle_error.is_some() {
                            handle_error = state_transition.handle_error();
                        }
                        if *state_transition.begin_state() == current_state && *state_transition.event() == event {
                            possible_states.push(state_transition);
                        }
                    }
                    if !possible_states.is_empty() {
                        let mut rng = rand::thread_rng();
                        let state_transition = possible_states[rng.gen_range(0..possible_states.len())];
                        let end_state = state_transition.end_state().clone();
                        if self.states.iter().any(|s| *s.begin_state() == end_state) {
                            if let Some(action) = state_transition.action() {
                                action();
                            }
                            self.current_state = end_state;
                            return;
                        } else {
                            println!("The goal state {} of event {} for state {} is not defined!", end_state, event, state_transition.begin_state());
                            return;
                        }
                    }
                    if let Some(parent_state) = self.parent.get(&current_state) {
                        current_state = parent_state.clone();
                    } else {
                        if let Some(handle_error) = handle_error {
                            handle_error();
                        }
                        break;
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_action {
    ($name:ident, $body:block) => {
        let $name: Option<Rc<Box<dyn Fn() -> ()>>> = Some(Rc::new(Box::new(|| $body)));
    };
}

#[macro_export]
macro_rules! define_state_and_parent {
    () => {
        (
            Vec::<
                Box<
                    dyn Transition<
                        State = String,
                        Event = String,
                        Action = Box<dyn Fn() -> ()>,
                        HandleError = Box<dyn Fn() -> ()>,
                    >,
                >,
            >::new(),
            HashMap::new()
        )
    };
}
