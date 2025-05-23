
use crate::*;

pub const SCREEN_SHAKE_TAU: f32 = 0.1;
pub const SCREEN_SHAKE_AMPLITUDE: f32 = 0.01;

pub struct GameState {
    pub logic_system: LogicSystem,
    pub state: UndoState,
    pub undo_stack: Vec<UndoState>,
    pub redo_stack: Vec<UndoState>,
    pub sequent_position: ScreenSize,
    pub sequent_scale: f32,
    pub last_shake_time: f32,
    pub initial_sequent: Sequent,
    pub finished_proof: bool,
}

#[derive(Clone)]
pub struct UndoState {
    pub proof: Proof,
    pub editing_formulas: bool,

    /// ID of the current focused formula field
    pub formulas_position: Option<u32>,

    /// Next id that will be assigned to empty fields, to make sure they are unique. This means all fields in proof will have an id below this.
    pub next_formula_index: u32,

    /// Next id that will be assigned to proof nodes, to make sure they are unique. This means all nodes in proof will have an id below this.
    pub next_proof_index: u32,

    /// Creation times of the fields in the sequent, indexed by their id
    pub fields_creation_time: HashMap<u32, f32>,
    
    pub node_to_check_after_fields_completed: Option<u32>,
}

pub fn game_frame(state: &mut State, app: &App, gfx: &mut Graphics, draw: &mut Draw) {

    let GameMode::Ingame(game_state) = &mut state.mode else { return };

    game_state.finished_proof = false;

    // Handle undo/redo
    if action::was_pressed(action::Action::Undo, state.settings.bindings(), app) {
        if !undo(game_state) {
            screen_shake(game_state, app.timer.elapsed_f32());
        }
    }
    else if action::was_pressed(action::Action::Redo, state.settings.bindings(), app) {
        if !redo(game_state) {
            screen_shake(game_state, app.timer.elapsed_f32());
        }
    }

    if action::was_pressed(action::Action::Restart, state.settings.bindings(), app) {
        record_undo_entry(game_state);
        game_state.state = get_start_sequent_state(game_state.initial_sequent.clone(), app.timer.elapsed_f32());
    }

    let special_mode = action::is_down(action::Action::SpecialRuleMode, state.settings.bindings(), app);

    if game_state.state.editing_formulas {
        match game_state.state.formulas_position {
            Some(position) => {
                // Check for operator insertion
                for (i, op) in game_state.logic_system.operators.clone().into_iter().enumerate() {
                    if action::was_pressed(action::Action::InsertOperator(i as u32), state.settings.bindings(), app) {
                        
                        record_undo_entry(game_state);
                        match proof::place_uncompleted_operator(op, position, &mut game_state.state.proof, &mut game_state.state.next_formula_index) {
                            Some(new_field) => game_state.state.formulas_position = Some(new_field),
                            None => exit_formula_mode(game_state, app.timer.elapsed_f32()),
                        }
                        
                        break;
                    } 
                }

                // Check for variable insertion
                for i in 0..MAX_VARIABLE_COUNT {
                    if action::was_pressed(action::Action::InsertVariable(i), state.settings.bindings(), app) {
                        
                        record_undo_entry(game_state);
                        match proof::place_variable(i, position, &mut game_state.state.proof) {
                            Some(new_field) => game_state.state.formulas_position = Some(new_field),
                            None => {
                                exit_formula_mode(game_state, app.timer.elapsed_f32());
                            },
                        }
                        
                        break;     
                    } 
                }

                // Previous and next fields
                if action::was_pressed(action::Action::NextField, state.settings.bindings(), app) {
                    game_state.state.formulas_position = Some(proof::formula_as_field(
                        proof::search_fields_by_id_in_proof(&mut game_state.state.proof, Some(position))[0]
                    ).next_id);
                }
                if action::was_pressed(action::Action::PreviousField, state.settings.bindings(), app) {
                    game_state.state.formulas_position = Some(proof::formula_as_field(
                        proof::search_fields_by_id_in_proof(&mut game_state.state.proof, Some(position))[0]
                    ).prev_id);
                }
            },
            None => {
                
            },
        }
        
    }
    else { // Not editing formulas

        game_state.state.fields_creation_time.clear(); // Should be ok since is 0(1) if empty (according to the code)

        let undo_entry = game_state.state.clone(); 

        match proof::get_first_unfinished_proof(&mut game_state.state.proof) {
            Some(current_proof) => {
                current_proof.last_focused_time = app.timer.elapsed_f32();

                // Check for rules insertion
                for i in 0..game_state.logic_system.rules.len() {
                    if action::was_pressed(action::Action::InsertRule(i as u32), state.settings.bindings(), app) {

                        let rule = if special_mode {
                            match &game_state.logic_system.special_rules[i] {
                                Some(rule) => rule,
                                None => &game_state.logic_system.rules[i],
                            }
                        }
                        else {
                            &game_state.logic_system.rules[i]
                        };

                        let (branches, field_count) = rule.as_ref().create_branches(&current_proof.root);
                        match branches {
                            Some(new_branches) => {
                                current_proof.branches = new_branches.into_iter().map(|s|
                                    proof::sequent_as_empty_proof(s, app.timer.elapsed_f32(), &mut game_state.state.next_proof_index)
                                ).collect();

                                current_proof.rule_id = Some(i as u32);
                                current_proof.rule_set_time = app.timer.elapsed_f32();

                                game_state.state.node_to_check_after_fields_completed = Some(current_proof.id);

                                add_undo_entry(undo_entry, game_state);
                            },
                            None => {
                                screen_shake(game_state, app.timer.elapsed_f32());
                            },
                        } 

                        if field_count > 0 {
                            game_state.state.next_formula_index = field_count;
                            game_state.state.formulas_position = Some(0);

                            game_state.state.editing_formulas = true;
                        }

                        break;
                    } 
                }
            },
            None => game_state.finished_proof = true, // TODO: show proof is finished
        }
    }

    // Draw the proof
    let shake_delta = get_shake_delta_position(game_state, app.timer.elapsed_f32());

    let mut render_info = proof::rendering::RenderInfo {
        draw,
        gfx,
        text_font: &state.text_font,
        symbol_font: &state.symbol_font,
        cached_sizes: &state.cached_sizes,
        focused_formula_field: game_state.state.formulas_position,
        editing_formulas: game_state.state.editing_formulas,
        logic_system: &game_state.logic_system,
        scale: game_state.sequent_scale,
        time: app.timer.elapsed_f32(),
        theme: *state.settings.theme(),
        focus_rect: ScreenRect::nothing(),
        fields_creation_time: &mut game_state.state.fields_creation_time,
    };

    let proof_width = get_proof_width(&game_state.state.proof, &mut render_info);
    let mut base_position = ScreenPosition {
        x: -proof_width * 0.5,
        y: -0.7,
    };

    base_position = base_position.add(shake_delta);
    base_position = base_position.add(game_state.sequent_position);

    draw_proof(&game_state.state.proof, base_position, &mut render_info);

    let focus_rect = render_info.focus_rect;
    adjust_proof_position(state.screen_ratio, proof_width, game_state, focus_rect, app);

    // Handle hide UI key
    if action::was_pressed(action::Action::ToggleKeys, state.settings.bindings(), app) {
        state.settings.set_show_game_keys(!state.settings.show_game_keys());
    }

    if *state.settings.show_game_keys() {
        game_ui::render_ui(special_mode, &state.symbol_font, draw, gfx, state);
    }

    // Handle exit key 
    if action::was_pressed(action::Action::Exit, state.settings.bindings(), app) {
        state.mode = menus::get_in_menu(menus::main_menu(state));
    }
}



fn record_undo_entry(gs: &mut GameState) {
    add_undo_entry(gs.state.clone(), gs);
}

fn add_undo_entry(entry: UndoState, gs: &mut GameState) {
    gs.undo_stack.push(entry);
    gs.redo_stack = Vec::new();
}

fn undo(gs: &mut GameState) -> bool {
    match gs.undo_stack.pop() {
        Some(mut last_state) => {
            std::mem::swap(&mut gs.state, &mut last_state);            
            gs.redo_stack.push(last_state);
            return true;
        },
        None => return false
    }
}

fn redo(gs: &mut GameState) -> bool {
    match gs.redo_stack.pop() {
        Some(mut last_state) => {
            std::mem::swap(&mut gs.state, &mut last_state);
            gs.undo_stack.push(last_state); 
            return true;
        },
        None => return false
    }
}

fn screen_shake(game_state: &mut GameState, time: f32) {
    game_state.last_shake_time = time;
}

fn get_shake_delta_position(game_state: &GameState, time: f32) -> ScreenSize {
    let t = time - game_state.last_shake_time;

    if t < 2.0 * SCREEN_SHAKE_TAU {
        let amplitude_attenuation = 1.0 - animation::ease_out_exp(t, SCREEN_SHAKE_TAU);
        let rand_centered = || { (notan::random::rand::random::<f32>() * 2.0 - 1.0) * amplitude_attenuation * SCREEN_SHAKE_AMPLITUDE };

        return ScreenSize {
            x: rand_centered(), 
            y: rand_centered()
        };
    }
    else {
        return ScreenSize { x: 0.0, y: 0.0 };
    }
}

fn adjust_proof_position(screen_ratio: f32, proof_width: f32, game_state: &mut GameState, focus_rect: ScreenRect, app: &App) {
    // If larger than screen move to center focused element, otherwise center the sequent
    let current_x_shift = if game_state.finished_proof  {
        game_state.sequent_position.x
    }
    else if focus_rect == ScreenRect::nothing() {
        0.0
    }
    else if proof_width > (screen_ratio - SEQUENT_SAFE_ZONE_SIDES) * 2.0 {
        focus_rect.center().x
    }
    else {
        game_state.sequent_position.x
    };

    /* Other possible behavior: if the focused element is near borders or outside screen, move sequent

    let safe_left = SEQUENT_SAFE_ZONE_SIDES - screen_ratio;
    let safe_right = - SEQUENT_SAFE_ZONE_SIDES + screen_ratio;

    let overflow_left = render_info.focus_rect.bottom_left.x < safe_left;
    let overflow_right = render_info.focus_rect.top_right.x > safe_right;

    let current_x_shift = if render_info.focus_rect == ScreenRect::nothing() {
        0.0
    }
    else if overflow_left && overflow_right {
        render_info.focus_rect.center().x
    }
    else if overflow_left {
        render_info.focus_rect.bottom_left.x - safe_left
    }
    else if overflow_right {
        render_info.focus_rect.top_right.x - safe_right
    }
    else {
        0.0
    };
    */

    let current_y_shift = if game_state.finished_proof  {
        game_state.sequent_position.y
    }
    else if focus_rect == ScreenRect::nothing() {
        0.0
    }
    else if -game_state.sequent_position.y + focus_rect.top_right.y > 1.0 - SEQUENT_SAFE_ZONE_TOP {
        focus_rect.top_right.y - 1.0 + SEQUENT_SAFE_ZONE_TOP
    }
    else {
        game_state.sequent_position.y
    };

    game_state.sequent_position.x -= CAMERA_MOVEMENT_SPEED_X * current_x_shift * app.timer.delta_f32();
    game_state.sequent_position.y -= CAMERA_MOVEMENT_SPEED_Y * current_y_shift * app.timer.delta_f32();

    let target_size = if game_state.finished_proof {
        f32::min(1.0, (screen_ratio - SEQUENT_SAFE_ZONE_SIDES) * 2.0 / proof_width * game_state.sequent_scale)
    } else {
        1.0
    };

    game_state.sequent_scale += (target_size - game_state.sequent_scale) * CAMERA_MOVEMENT_SPEED_SCALE * app.timer.delta_f32();
}

pub fn get_initial_state(start_seq: Sequent, time: f32) -> GameMode {
    return GameMode::Ingame(ingame::GameState {
        logic_system: proof::natural_logic::get_system(),
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
        state: get_start_sequent_state(start_seq.clone(), time),
        sequent_position: ScreenSize::zero(),
        sequent_scale: 1.0,
        last_shake_time: f32::NEG_INFINITY,
        initial_sequent: start_seq,
        finished_proof: false,
    });
}

fn get_start_sequent_state(s: Sequent, time: f32) -> UndoState {
    let mut next_proof_id = 0;
    let mut proof = sequent_as_empty_proof(s, time, &mut next_proof_id);
    let mut fields = proof::search_fields_by_id_in_proof(&mut proof, None);

    if fields.len() == 0 {
        return UndoState {
            editing_formulas: false,
            formulas_position: None,
            next_formula_index: 0,
            fields_creation_time: HashMap::with_capacity(20),
            proof: proof.clone(),
            node_to_check_after_fields_completed: None,
            next_proof_index: next_proof_id,
        };
    }
    else {
        return UndoState {
            editing_formulas: true,
            formulas_position: Some(formula_as_field(fields[0]).id),
            next_formula_index: fields.iter_mut().map(|f| { formula_as_field(f).id }).max().unwrap() + 1,
            fields_creation_time: HashMap::with_capacity(20),
            proof: proof.clone(),
            node_to_check_after_fields_completed: None,
            next_proof_index: next_proof_id,
        };
    }
}


/// Exists formula mode, and check that the fields are correct for the new rule. 
fn exit_formula_mode(game_state: &mut GameState, time: f32) {
    match &mut game_state.state.node_to_check_after_fields_completed {
        Some(proof_id) => {
            let proof = get_proof_node_by_id(&mut game_state.state.proof, *proof_id).unwrap();
            let ok = game_state.logic_system.rules[proof.rule_id.unwrap() as usize].check_validity(proof);

            if ok {
                game_state.state.node_to_check_after_fields_completed = None;
                game_state.state.editing_formulas = false; // Correct -> exit formula mode
            }
            else {
                // Incorrect
                game_state.state.formulas_position = None;
                proof.is_rule_invalid = true;
                screen_shake(game_state, time);
            }
        },
        None => game_state.state.editing_formulas = false, // Nothing to check -> exit formula mode
    }
}
