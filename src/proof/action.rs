
use crate::KeyCode;
use crate::Keyboard;
use crate::OperatorType;
use std::collections::HashMap;

// To be completed with accual action douable by the player
#[derive(Clone)]
#[derive(Copy)]
pub enum Action {
    NoAc,
    Pause,
    Operation(OperatorType),
}

impl Action{

    // Calls all the functions that computes action started by the player
    pub fn perform(act: Action){
        match act {
            Action::NoAc => unreachable!(),
            Action::Pause => unreachable!(), // TODO: function that open the pause menu
            Action::Operation(op) => unreachable!(), // TODO: Function that can fill the curent sequent
        }
    }
}


// Only put useful Keycodes in here
static KEYCODES: [KeyCode; 1] = [KeyCode::Escape];

// Look for imput from the player in the last frame and execut the coresponding action
pub fn get_input(keyboard: Keyboard, bindings: HashMap<KeyCode, Action>){

    for code in KEYCODES.iter(){
        if keyboard.was_pressed(*code){
            Action::perform(*bindings.get(code).unwrap());
        }
    }
}

