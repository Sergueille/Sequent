

use std::collections::HashMap;

use notan::draw::{Draw, DrawShapes, DrawTextSection};
use notan::app::Graphics;
use notan::prelude::*;
use crate::coord::PixelPosition;
use crate::parser::Level;
use crate::{action, ingame, misc, proof};
use crate::State;
use crate::coord::*;

pub const TEXT_SIZE: f32 = 40.0;
pub const LABEL_HEIGHT: f32 = 0.07;
pub const LINE_GAP: f32 = 0.02;

pub const BUTTON_HEIGHT: f32 = 0.1;
pub const BUTTONS_WIDTH: f32 = 0.8;
pub const BUTTONS_PADDING: f32 = 0.02;
pub const BUTTONS_PADDING_FOCUSED: f32 = 0.05;

pub const BUTTONS_FLASH_TAU: f32 = 0.1;
pub const BUTTONS_COLOR_TAU: f32 = 0.1;
pub const BUTTONS_PADDING_TAU: f32 = 0.07;

pub const LEVEL_SELECTION_HEIGHT: f32 = 0.15;
pub const LEVEL_SELECTION_WIDTH: f32 = 1.2;
pub const LEVEL_SELECTION_MARGIN_AFTER_NAME: f32 = 0.05;
pub const LEVEL_SELECTION_DIFFICULTY_TEXT_SIZE: f32 = 20.0;

pub const MENU_LEFT_SPACE: f32 = 0.2;
pub const MENUS_BASE_Y: f32 = 0.1;
pub const MENUS_Y_SCROLL_TAU: f32 = 0.2;
pub const MENUS_Y_SCROLL_SAFE_ZONE: f32 = 0.5;

pub struct MenuState {
    pub current_menu: Menu,

    /// Index of the focused element in the menu, ignoring non focusable elements.
    /// If focused_element is n, there is n focusable elements before the currently focused element.
    pub focused_element: usize,

    /// Screen size. 0.0 is top of the menu 
    pub y_scroll: f32,
}

#[derive(Clone)]
pub struct KeyRecordState {
    pub next_menu: fn(&State) -> Menu,
    pub focused_element: usize,
    pub y_scroll: f32,
    pub action: action::Action,
}

/// All info needed to draw a menu
pub struct DrawInfo<'a> {
    pub screen_ratio: f32,
    pub draw: &'a mut Draw,
    pub gfx: &'a mut Graphics,
    pub app: &'a mut App,
    pub theme: crate::settings::Theme,
    pub text_font: &'a crate::Font,
    pub symbol_font: &'a crate::Font,
    pub bindings: &'a crate::action::Bindings,
    pub cached_sizes: &'a HashMap<char, f32>
}

#[derive(Clone, Copy)]
pub enum MenuEffect {
    ChangeGameMode(fn() -> crate::GameMode),
    ChangeMenu(fn(&State) -> Menu),
    SetActionKey(action::Action),
    Quit,
    Nothing
}

pub struct Menu {
    pub elements: Vec<Box<dyn MenuItem>>,
    pub previous_menu: Option<fn(&State) -> Menu>
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

pub struct LevelSelection {
    pub level: Level,
    pub last_focused_time: f32,
    pub last_unfocused_time: f32,
}

pub struct Label {
    pub label: String,
}

impl MenuItem for Button {
    fn draw(&mut self, bottom_left: ScreenPosition, focused: bool, info: &mut DrawInfo) {
        handle_focus_times(&mut self.last_focused_time, &mut self.last_unfocused_time, focused, info);

        let size = ScreenSize { x: BUTTONS_WIDTH, y: BUTTON_HEIGHT };

        let bg_color = get_bg_color(self.last_focused_time, self.last_unfocused_time, focused, info);
        let left_padding = get_left_padding(self.last_focused_time, self.last_unfocused_time, focused, info);

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

impl MenuItem for LevelSelection {
    fn draw(&mut self, bottom_left: ScreenPosition, focused: bool, info: &mut DrawInfo) {
        handle_focus_times(&mut self.last_focused_time, &mut self.last_unfocused_time, focused, info);

        let size = ScreenSize { x: LEVEL_SELECTION_WIDTH, y: LEVEL_SELECTION_HEIGHT };

        let bg_color = get_bg_color(self.last_focused_time, self.last_unfocused_time, focused, info);
        let left_padding = get_left_padding(self.last_focused_time, self.last_unfocused_time, focused, info);

        info.draw.rect(bottom_left.to_pixel(info.gfx).as_couple(), size.to_pixel(info.gfx))
            .color(bg_color);

        let mut text_pos = bottom_left;
        text_pos.x += left_padding;
        text_pos.y += BUTTON_HEIGHT * 0.5;

        {
            let mut text = info.draw.text(info.text_font, &self.level.name);
            
            text.position(text_pos.to_pixel(info.gfx).x, text_pos.to_pixel(info.gfx).y)
                .v_align_middle()
                .h_align_left();
        
            set_text_size(&mut text, TEXT_SIZE, info.gfx);
        }

        let text_width = PixelPosition::from_couple((info.draw.last_text_bounds().width, 0.0)).to_screen(&info.gfx).difference_with(
            PixelPosition::from_couple((0.0, 0.0)).to_screen(&info.gfx)
        ).x;

        text_pos.x += text_width + LEVEL_SELECTION_MARGIN_AFTER_NAME;
        text_pos.y -= BUTTON_HEIGHT * 0.5;

        let mut render_info = proof::rendering::RenderInfo {
            draw: info.draw,
            gfx: info.gfx,
            text_font: info.text_font,
            symbol_font: info.symbol_font,
            cached_sizes: info.cached_sizes,
            focused_formula_field: None,
            editing_formulas: false,
            logic_system: &proof::natural_logic::get_system(),
            scale: 1.0,
            time: 0.0,
            theme: info.theme,
            focus_rect: ScreenRect::nothing(),
            fields_creation_time: &mut HashMap::new(),
        };

        proof::rendering::draw_sequent(&self.level.seq, text_pos, 1.0, &mut render_info);
        let width = proof::rendering::get_sequent_width(&self.level.seq, &mut render_info);

        text_pos.x += width + LEVEL_SELECTION_MARGIN_AFTER_NAME;
        text_pos.y += BUTTON_HEIGHT * 0.5;

        {
            let diff_string = format!("({})", self.level.difficulty.to_string());
            let mut text = info.draw.text(info.text_font, &diff_string);
            
            text.position(text_pos.to_pixel(info.gfx).x, text_pos.to_pixel(info.gfx).y)
                .v_align_middle()
                .h_align_left()
                .color(info.theme.ui_text_dark);
        
            set_text_size(&mut text, LEVEL_SELECTION_DIFFICULTY_TEXT_SIZE, info.gfx);
        }
    }

    fn on_interact(&mut self) {
        // TODO
    }

    fn get_effect(&self) -> MenuEffect { 
        MenuEffect::Nothing
    }

    fn get_height(&self, _info: &mut DrawInfo) -> f32 {
        LEVEL_SELECTION_HEIGHT
    }

    fn get_focusable(&self) -> bool {
        true
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
        symbol_font: &state.symbol_font,
        bindings: state.settings.bindings(),
        cached_sizes: &state.cached_sizes,
    };

    let mut position = ScreenPosition { 
        x: MENU_LEFT_SPACE - info.screen_ratio,
        y: MENUS_BASE_Y - menu_state.y_scroll,
    };

    let mut nb_focusable = 0;
    let mut focusable_y = None;

    // Draw elements
    for element in menu_state.current_menu.elements.iter_mut() {
        position.y -= element.get_height(&mut info);

        if element.get_focusable() {
            element.draw(position, nb_focusable == menu_state.focused_element, &mut info);        

            if nb_focusable == menu_state.focused_element {
                focusable_y = Some(position.y);
            }

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

    // Scroll
    let scroll_amount = match focusable_y {
        Some(y) => if y > MENUS_Y_SCROLL_SAFE_ZONE {
            y - MENUS_Y_SCROLL_SAFE_ZONE
        } else if y < -MENUS_Y_SCROLL_SAFE_ZONE {
            y + MENUS_Y_SCROLL_SAFE_ZONE 
        }
        else {
            0.0
        },
        None => 0.0,
    };

    menu_state.y_scroll += scroll_amount * info.app.timer.delta_f32() / MENUS_Y_SCROLL_TAU;
    if menu_state.y_scroll > 0.0 {
        menu_state.y_scroll = 0.0;
    }

    let previous_menu = menu_state.current_menu.previous_menu; // Copy this value now to allow mutating the state

    // Check for interactions
    let mut nb_focusable = 0;
    for element in menu_state.current_menu.elements.iter_mut() {
        if element.get_focusable() {
            if nb_focusable == menu_state.focused_element && action::was_pressed(action::Action::Confirm, info.bindings, info.app) {
                element.on_interact();

                match element.get_effect() {
                    MenuEffect::ChangeGameMode(mode) => state.mode = mode(),
                    MenuEffect::ChangeMenu(menu) => {
                        state.mode = get_in_menu(menu(state));
                    },
                    MenuEffect::Quit => {
                        info.app.exit();
                    },
                    MenuEffect::SetActionKey(action) => {
                        state.mode = crate::GameMode::SetKey(KeyRecordState {
                            next_menu: keyboard,
                            focused_element: menu_state.focused_element,
                            y_scroll: menu_state.y_scroll,
                            action,
                        });
                    }
                    MenuEffect::Nothing => { },
                }

                break;
            }

            nb_focusable += 1;
        }
    }

    // Check for back key
    if action::was_pressed(action::Action::Exit, state.settings.bindings(), info.app) {
        match previous_menu {
            Some(get_menu) => {
                state.mode = get_in_menu(get_menu(state));
            },
            None => { }
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

pub fn main_menu(_: &State) -> Menu {
    return Menu { 
        elements: vec![
            button("Solve", MenuEffect::ChangeMenu(level_list)),
            button("Free editing", MenuEffect::ChangeGameMode(start_free_editing)),
            button("Settings", MenuEffect::ChangeMenu(settings)),
            button("Quit", MenuEffect::ChangeMenu(quit_confirmation)),
        ], 
        previous_menu: None,
    };
}

pub fn level_list(state: &State) -> Menu {
    let mut levels_buttons: Vec<Box<dyn MenuItem>> = state.levels.iter().map(
        |level| Box::new(LevelSelection { level: level.clone(), last_focused_time: 0.0, last_unfocused_time: 0.0 }) as Box<dyn MenuItem>
    ).collect();

    levels_buttons.push(button("Back", MenuEffect::ChangeMenu(main_menu)));

    return Menu { 
        elements: levels_buttons, 
        previous_menu: Some(main_menu),
    };
}

pub fn settings(_: &State) -> Menu {
    return Menu { 
        elements: vec![
            label("Settings"),
            button("Keyboard", MenuEffect::ChangeMenu(keyboard)),
            button("Back", MenuEffect::ChangeMenu(main_menu))
        ],
        previous_menu: Some(main_menu),
    };
}

pub fn keyboard(state: &State) -> Menu {
    let mut pairs = state.settings.bindings().iter()
        .filter(|(action, _)| {
            **action != action::Action::NoAc
        })
        .collect::<Vec<(&action::Action, &KeyCode)>>();

    pairs.sort_by(|(a, _), (b, _)| {
        action::get_action_name(**a).cmp(&action::get_action_name(**b))
    });

    let mut keys = pairs.into_iter().map(|(action, key_name)| {
        button(
            &format!("{}: {}", action::get_action_name(*action), action::key_code_display(*key_name)),
            MenuEffect::SetActionKey(*action),
        ) as Box<dyn MenuItem>
    }).collect();

    let mut res: Vec<Box<dyn MenuItem>> = vec![
        label("Keyboard configuration"),
    ];

    res.append(&mut keys);

    res.push(button("Back", MenuEffect::ChangeMenu(settings)));


    return Menu { 
        elements: res,
        previous_menu: Some(settings)
    };
}

pub fn quit_confirmation(_: &State) -> Menu {
    return Menu { 
        elements: vec![
            label("Confirm exiting the app?"),
            button("No", MenuEffect::ChangeMenu(main_menu)),
            button("Yes", MenuEffect::Quit)
        ], 
        previous_menu: Some(main_menu),
    };
}

pub fn get_in_menu(m: Menu) -> crate::GameMode {
    return crate::GameMode::Menu(MenuState { current_menu: m, focused_element: 0, y_scroll: 0.0 });
}

pub fn handle_key_input(key_record_state: KeyRecordState, state: &mut State, draw: &mut Draw, gfx: &Graphics, app: &App) {
    let center = ScreenPosition::center();

    let text_content = format!("Press a key to assign it to: {}", action::get_action_name(key_record_state.action));
    let mut text = draw.text(&state.text_font, &text_content);
    text.position(center.to_pixel(gfx).x, center.to_pixel(gfx).y)
        .v_align_middle()
        .h_align_center();

    set_text_size(&mut text, 60.0, gfx);

    match action::which_key_pressed(app) {
        Some(key) => {
            let mut new_bindings = state.settings.bindings().clone();
            new_bindings.insert(key_record_state.action, key);
            
            state.settings.set_bindings(new_bindings);

            state.mode = crate::GameMode::Menu(MenuState { 
                current_menu: (key_record_state.next_menu)(state), 
                focused_element: key_record_state.focused_element, 
                y_scroll: key_record_state.y_scroll, 
            });
        },
        None => { },
    }
}

fn start_free_editing() -> crate::GameMode {
    return ingame::get_initial_state(proof::get_empty_sequent(), 0.0);
}

fn handle_focus_times(last_focused_time: &mut f32, last_unfocused_time: &mut f32, focused: bool, info: &mut DrawInfo) {
    let time = info.app.timer.elapsed_f32();
    if focused {
        *last_focused_time = time;
    }
    else {
        *last_unfocused_time = time;
    }
}

fn get_bg_color(last_focused_time: f32, last_unfocused_time: f32, focused: bool, info: &mut DrawInfo) -> Color {
    let time = info.app.timer.elapsed_f32();
    if focused {
        let t = crate::animation::ease_out_exp(time - last_unfocused_time, BUTTONS_FLASH_TAU);
        return misc::color_lerp(info.theme.ui_button_flash, info.theme.ui_button_focus, t);
    } else {
        let t = crate::animation::ease_out_exp(time - last_focused_time, BUTTONS_FLASH_TAU);
        return misc::color_lerp(info.theme.ui_button_focus, info.theme.ui_button, t);
    }
}

fn get_left_padding(last_focused_time: f32, last_unfocused_time: f32, focused: bool, info: &mut DrawInfo) -> f32 {
    let time = info.app.timer.elapsed_f32();
    if focused {
        let t = crate::animation::ease_out_exp(time - last_unfocused_time, BUTTONS_PADDING_TAU);
        return misc::lerp(BUTTONS_PADDING, BUTTONS_PADDING_FOCUSED, t);
    } else {
        let t = crate::animation::ease_out_exp(time - last_focused_time, BUTTONS_PADDING_TAU);
        return misc::lerp(BUTTONS_PADDING_FOCUSED, BUTTONS_PADDING, t);
    }
}


