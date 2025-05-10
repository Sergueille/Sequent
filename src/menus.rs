

use notan::draw::{Draw, DrawShapes, DrawTextSection};
use notan::app::Graphics;
use notan::prelude::App;
use crate::{action, ingame, misc, proof};
use crate::State;
use crate::coord::*;

pub const TEXT_SIZE: f32 = 40.0;
pub const LABEL_HEIGHT: f32 = 0.07;
pub const LINE_GAP: f32 = 0.02;

pub const BUTTON_HEIGHT: f32 = 0.1;
pub const BUTTONS_WIDTH: f32 = 0.7;
pub const BUTTONS_PADDING: f32 = 0.02;
pub const BUTTONS_PADDING_FOCUSED: f32 = 0.05;

pub const BUTTONS_FLASH_TAU: f32 = 0.1;
pub const BUTTONS_COLOR_TAU: f32 = 0.1;
pub const BUTTONS_PADDING_TAU: f32 = 0.07;

pub const MENU_LEFT_SPACE: f32 = 0.2;
pub const MENUS_BASE_Y: f32 = 0.0;

pub struct MenuState {
    pub current_menu: Menu,

    /// Index of the focused element in the menu, ignoring non focusable elements.
    /// If focused_element is n, there is n focusable elements before the currently focused element.
    pub focused_element: usize,
}

/// All info needed to draw a menu
pub struct DrawInfo<'a> {
    pub screen_ratio: f32,
    pub draw: &'a mut Draw,
    pub gfx: &'a mut Graphics,
    pub app: &'a mut App,
    pub theme: crate::settings::Theme,
    pub text_font: &'a crate::Font,
    pub bindings: &'a crate::action::Bindings,
}

#[derive(Clone, Copy)]
pub enum MenuEffect {
    ChangeGameMode(fn() -> crate::GameMode),
    ChangeMenu(fn() -> Menu),
    Quit,
    Nothing
}

pub struct Menu {
    pub elements: Vec<Box<dyn MenuItem>>,
}

/// An element in a menu (button, label, etc...)
pub trait MenuItem {
    fn draw(&mut self, bottom_left: ScreenPosition, focused: bool, info: &mut DrawInfo);
    /// Called by the menu system when the user pressed confirm while focusing the element
    fn on_interact(&mut self);
    /// What should the element do when the user press confirm? Will be executed after on_interact
    fn get_effect(&self) -> MenuEffect;
    fn get_height(&self, info: &mut DrawInfo) -> f32;
    fn get_focusable(&self) -> bool;
}

pub struct Button {
    pub label: String,
    pub last_focused_time: f32,
    pub last_unfocused_time: f32,
    pub on_press: MenuEffect,
}

pub struct Label {
    pub label: String,
}

impl MenuItem for Button {
    fn draw(&mut self, bottom_left: ScreenPosition, focused: bool, info: &mut DrawInfo) {
        let time = info.app.timer.elapsed_f32();
        if focused {
            self.last_focused_time = time;
        }
        else {
            self.last_unfocused_time = time;
        }

        let size = ScreenSize { x: BUTTONS_WIDTH, y: BUTTON_HEIGHT };

        let bg_color = if focused {
            let t = crate::animation::ease_out_exp(time - self.last_unfocused_time, BUTTONS_FLASH_TAU);
            misc::color_lerp(info.theme.ui_button_flash, info.theme.ui_button_focus, t)
        } else {
            let t = crate::animation::ease_out_exp(time - self.last_focused_time, BUTTONS_FLASH_TAU);
            misc::color_lerp(info.theme.ui_button_focus, info.theme.ui_button, t)
        };

        let left_padding = if focused {
            let t = crate::animation::ease_out_exp(time - self.last_unfocused_time, BUTTONS_PADDING_TAU);
            misc::lerp(BUTTONS_PADDING, BUTTONS_PADDING_FOCUSED, t)
        } else {
            let t = crate::animation::ease_out_exp(time - self.last_focused_time, BUTTONS_PADDING_TAU);
            misc::lerp(BUTTONS_PADDING_FOCUSED, BUTTONS_PADDING, t)
        };

        info.draw.rect(bottom_left.to_pixel(info.gfx).as_couple(), size.to_pixel(info.gfx))
            .color(bg_color);

        let mut text_pos = bottom_left;
        text_pos.x += left_padding;
        text_pos.y += BUTTON_HEIGHT * 0.5;

        let mut text = info.draw.text(info.text_font, &self.label);
            
        text.position(text_pos.to_pixel(info.gfx).x, text_pos.to_pixel(info.gfx).y)
            .v_align_middle()
            .h_align_left();

        set_text_size(&mut text, TEXT_SIZE, info.gfx);
    }

    fn on_interact(&mut self) {
        // TODO
    }

    fn get_effect(&self) -> MenuEffect { 
        self.on_press
    }

    fn get_height(&self, _info: &mut DrawInfo) -> f32 {
        BUTTON_HEIGHT
    }

    fn get_focusable(&self) -> bool {
        true
    }
}

impl MenuItem for Label {
    fn draw(&mut self, bottom_left: ScreenPosition, _focused: bool, info: &mut DrawInfo) {
        let mut center = bottom_left;
        center.y += LABEL_HEIGHT * 0.5;

        let mut text = info.draw.text(info.text_font, &self.label);

        text.position(center.to_pixel(info.gfx).x, center.to_pixel(info.gfx).y)
            .v_align_middle()
            .h_align_left();

        set_text_size(&mut text, TEXT_SIZE, info.gfx);
    }

    fn on_interact(&mut self) { }

    fn get_effect(&self) -> MenuEffect { MenuEffect::Nothing }

    fn get_height(&self, _info: &mut DrawInfo) -> f32 {
        LABEL_HEIGHT
    }

    fn get_focusable(&self) -> bool {
        false
    }
}

/// Draw and update the menu (change focus, animate elements, call callbacks...) for the frame
pub fn draw_menu(state: &mut State, app: &mut App, gfx: &mut Graphics, draw: &mut Draw) {
    let crate::GameMode::Menu(menu_state) = &mut state.mode else { unreachable!() }; 

    let mut info = DrawInfo {
        app,
        screen_ratio: state.screen_ratio,
        draw,
        gfx,
        theme: *state.settings.theme(),
        text_font: &state.text_font,
        bindings: state.settings.bindings(),
    };

    let mut position = ScreenPosition { 
        x: MENU_LEFT_SPACE - info.screen_ratio,
        y: MENUS_BASE_Y,
    };

    let mut nb_focusable = 0;

    // Draw elements
    for element in menu_state.current_menu.elements.iter_mut() {
        position.y -= element.get_height(&mut info);

        if element.get_focusable() {
            element.draw(position, nb_focusable == menu_state.focused_element, &mut info);            
            nb_focusable += 1;
        }
        else {
            element.draw(position, false, &mut info);
        }

        position.y -= LINE_GAP;
    }

    // Change focused element
    if action::was_pressed(action::Action::Down, state.settings.bindings(), info.app) {
        menu_state.focused_element += 1;
    }
    else if action::was_pressed(action::Action::Up, state.settings.bindings(), info.app) {
        menu_state.focused_element += nb_focusable - 1;
    }

    menu_state.focused_element %= nb_focusable;

    // Check for interactions
    let mut nb_focusable = 0;
    for element in menu_state.current_menu.elements.iter_mut() {
        if element.get_focusable() {
            if nb_focusable == menu_state.focused_element && action::was_pressed(action::Action::Confirm, info.bindings, info.app) {
                element.on_interact();

                match element.get_effect() {
                    MenuEffect::ChangeGameMode(mode) => state.mode = mode(),
                    MenuEffect::ChangeMenu(menu) => {
                        state.mode = get_in_menu(menu());
                    },
                    MenuEffect::Quit => {
                        app.exit();
                    },
                    MenuEffect::Nothing => { },
                }

                break;
            }

            nb_focusable += 1;
        }
    }
}


fn button(label: &str, on_press: MenuEffect) -> Box<Button> {
    return Box::new(Button {
        label: label.to_string(),
        last_focused_time: f32::NEG_INFINITY,
        last_unfocused_time: f32::NEG_INFINITY,
        on_press,
    });
}

fn label(label: &str) -> Box<Label> {
    return Box::new(Label {
        label: label.to_string(),
    });
}

pub fn main_menu() -> Menu {
    return Menu { 
        elements: vec![
            button("Free editing", MenuEffect::ChangeGameMode(start_free_editing)),
            button("Settings", MenuEffect::ChangeMenu(settings)),
            button("Quit", MenuEffect::ChangeMenu(quit_confirmation)),
        ], 
    };
}

pub fn settings() -> Menu {
    return Menu { 
        elements: vec![
            label("Settings"),
            button("Keyboard", MenuEffect::ChangeMenu(keyboard)),
            button("Back", MenuEffect::ChangeMenu(main_menu)),
        ], 
    };
}

pub fn keyboard() -> Menu {
    return Menu { 
        elements: vec![
            label("Keyboard configuration: TODO :)"), // TODO
            button("Back", MenuEffect::ChangeMenu(settings)),
        ], 
    };
}

pub fn quit_confirmation() -> Menu {
    return Menu { 
        elements: vec![
            label("Confirm exiting the app?"),
            button("No", MenuEffect::ChangeMenu(main_menu)),
            button("Yes", MenuEffect::Quit)
        ], 
    };
}

pub fn get_in_menu(m: Menu) -> crate::GameMode {
    return crate::GameMode::Menu(MenuState { current_menu: m, focused_element: 0 });
}

fn start_free_editing() -> crate::GameMode {
    return ingame::get_initial_state(proof::get_empty_sequent(), 0.0);
}

