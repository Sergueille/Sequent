
use std::collections::HashMap;
use notan::app::App;

use crate::KeyCode;

pub type Bindings = HashMap<Action, KeyCode>;

/// Only put useful Keycodes in here
static KEYCODES: [KeyCode; 1] = [
    KeyCode::Escape
];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    NoAc,

    /// Corresponds to an operator `slot` that will assigned by the proof system
    /// This way, the keys are the same even if a different logic system is used
    InsertOperator(u32),    
    InsertVariable(u32),
    InsertRule(u32),
    SpecialRuleMode,

    NextField,
    PreviousField,

    Undo,
    Redo,

    ToggleKeys,

    Exit,
}

pub fn get_default_bindings() -> Bindings {
    let mut res = HashMap::new();

    let op_keys = [
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

    // Default for variables
    let var_keys = [
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
    }

    // Default for rules
    let var_keys = [
        KeyCode::A,
        KeyCode::Q,
        KeyCode::Z,
        KeyCode::S,
        KeyCode::E,
        KeyCode::D,
        KeyCode::R,
        KeyCode::F,
        KeyCode::T,
        KeyCode::G,
        KeyCode::Y,
        KeyCode::H,
        KeyCode::U,
    ];

    for (i, key) in var_keys.iter().enumerate() {
        res.insert(Action::InsertRule(i as u32), *key);
    }

    res.insert(Action::SpecialRuleMode, KeyCode::LShift);

    res.insert(Action::NextField, KeyCode::Right);
    res.insert(Action::PreviousField, KeyCode::Left);

    res.insert(Action::Undo, KeyCode::W);
    res.insert(Action::Redo, KeyCode::X);

    res.insert(Action::ToggleKeys, KeyCode::F1);

    res.insert(Action::Exit, KeyCode::Escape);

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

pub fn is_down(action: Action, bindings: &HashMap<Action, KeyCode>, app: &App) -> bool {
    match bindings.get(&action) {
        Some(key) => {
            return app.keyboard.is_down(*key);
        },
        None => {
            println!("No binding for action {:?}", action);
            return false;
        },
    }
}

pub fn key_code_display(code: KeyCode) -> String {
    match code {
        KeyCode::Escape => String::from("Esc"),
        _ => format!("{:?}", code),
    }
}
