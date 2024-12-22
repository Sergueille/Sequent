
use std::collections::HashMap;
use notan::app::App;

use crate::KeyCode;

/// Only put useful Keycodes in here
static KEYCODES: [KeyCode; 1] = [
    KeyCode::Escape
];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    NoAc,
    Pause,

    /// Corresponds to an operator `slot` that will assigned by the proof system
    /// This way, the keys are the same even if a different logic system is used
    InsertOperator(u32),
    
    InsertVariable(u32),
    InsertRule(u32),

    NextField,
    PreviousField,
}

pub fn get_default_bindings() -> HashMap<Action, KeyCode> {
    let mut res = HashMap::new();

    let op_keys = vec![
        KeyCode::Q,
        KeyCode::S,
        KeyCode::D,
        KeyCode::F,
        KeyCode::G,
        KeyCode::H,
        KeyCode::J,
        KeyCode::K,
    ];

    for (i, key) in op_keys.iter().enumerate() {
        res.insert(Action::InsertOperator(i as u32), *key);
    }

    // Default for variables and rules
    let var_keys = vec![
        KeyCode::A,
        KeyCode::Z,
        KeyCode::E,
        KeyCode::R,
        KeyCode::T,
        KeyCode::Y,
        KeyCode::U,
        KeyCode::I,
        KeyCode::O,
        KeyCode::P,
    ];

    for (i, key) in var_keys.iter().enumerate() {
        res.insert(Action::InsertVariable(i as u32), *key);
        res.insert(Action::InsertRule(i as u32), *key);
    }

    res.insert(Action::NextField, KeyCode::Return);
    res.insert(Action::PreviousField, KeyCode::RShift);

    return res;
}

pub fn was_pressed(action: Action, bindings: &HashMap<Action, KeyCode>, app: &App) -> bool {
    match bindings.get(&action) {
        Some(key) => {
            return app.keyboard.was_pressed(*key);
        },
        None => {
            println!("No binding for action {:?}", action);
            return false;
        },
    }
}

