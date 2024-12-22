
#![allow(dead_code)]

pub mod rendering;
pub mod calcul;
pub mod natural_logic;

type Variable = u32;

pub const MAX_VARIABLE_COUNT: u32 = 10;

/// Each rule will be a dedicated type that implement this.
pub trait Rule {
    /// Create proof template from the sequent. Returns None if not compatible.
    /// next_field_id should be increased if new empty fields are created. Otherwise, it must not be modified.
    fn create_branches(&self, root: &Sequent, next_field_id: &mut u32) -> Option<Vec<Proof>>; 
    /// Check if the sequents above the root of the proofs corresponds to the rule.
    fn check_validity(&self, proof: &Proof) -> bool; 
    /// Text to be displayed to the right of the horizontal bar.
    fn display_text(&self) -> &'static str; 
}

/// A proof tree.
#[derive(Clone)]
pub struct Proof {
    pub root: Sequent,
    pub branches: Vec<Proof>,
    pub rule_id: Option<u32>,
}

/// A sequent!
/// 
/// I used vec for both sides, will be useful if we want to implement other logic systems.
#[derive(Clone)]
pub struct Sequent {
    pub before: Vec<Formula>,
    pub after: Vec<Formula>,

    pub cached_text_section: Option<notan::glyph::Section<'static>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorType {
    Not, Impl, And, Or, Top, Bottom
}

/// arg1 and arg2 are None if the arity is 0 or 1.
/// Non-variable constants (top, bottom) are operators with arity 0.
#[derive(Clone, PartialEq, Eq)]
pub struct Operator {
    pub operator_type: OperatorType,
    pub arg1: Option<Box<Formula>>,
    pub arg2: Option<Box<Formula>>
}

#[derive(Clone, PartialEq, Eq)]
pub enum Formula {
    Operator(Operator),
    Variable(Variable),
    
    /// Corresponds to a zone that need to be completed by the player
    /// Blank spaces in formula have their own id. If will be used to place the cursor of the user.
    NotCompleted(FormulaField),
}

#[derive(Clone, PartialEq, Eq)]
pub struct FormulaField {
    pub id: u32,
    pub next_id: u32,
    pub prev_id: u32,
}

pub struct LogicSystem {
    pub operators: Vec<OperatorType>,
    pub rules: Vec<Box<dyn Rule>>,
}

fn get_operator_arity(op: OperatorType) -> u32 {
    match op {
        OperatorType::Not => 1,
        OperatorType::Impl => 2,
        OperatorType::And => 2,
        OperatorType::Or => 2,
        OperatorType::Top => 0,
        OperatorType::Bottom => 0,
    }
}

fn get_operator_symbol(op: OperatorType) -> &'static str {
    match op {
        OperatorType::Not => "¬",
        OperatorType::Impl => "→",
        OperatorType::And => "∧",
        OperatorType::Or => "∨",
        OperatorType::Top => "⊤",
        OperatorType::Bottom => "⊥",
    }
}

/// Smaller number means more priority. Use 0 for constants.
fn get_operator_priority(op: OperatorType) -> f32 {
    match op {
        OperatorType::Not => 1.0,
        OperatorType::Impl => 3.0,
        OperatorType::And => 2.0,
        OperatorType::Or => 2.0,
        OperatorType::Top => 0.0,
        OperatorType::Bottom => 0.0,
    }
}

/// Create a variable, then places it in field with field_id. Returns the id of the next field to be focused, if there is any left. 
pub fn place_variable(var: Variable, field_id: u32, proof: &mut Proof) -> Option<u32> {
    let field_formula = search_field_id_in_proof(proof, Some(field_id)).unwrap();
    let field = formula_as_field(field_formula).clone();

    let new_formula = Formula::Variable(var);

    *field_formula = new_formula;

    if field.next_id == field_id {
        return None;
    }
    else {
        formula_as_field(search_field_id_in_proof(proof, Some(field.prev_id)).unwrap()).next_id = field.next_id;
        formula_as_field(search_field_id_in_proof(proof, Some(field.next_id)).unwrap()).prev_id = field.prev_id;

        return Some(field.next_id);
    }
}

/// Create an operator with NotCompleted as arguments, then places it in field with field_id. Returns the id of the next field to be focused, if there is any left. 
pub fn place_uncompleted_operator(op: OperatorType, field_id: u32, proof: &mut Proof, next_index: &mut u32) -> Option<u32> {
    let arity = get_operator_arity(op);

    let field_formula = search_field_id_in_proof(proof, Some(field_id)).unwrap();
    let field = formula_as_field(field_formula).clone();

    let next_id;
    let new_formula;

    if arity == 0 {
        new_formula = Formula::Operator(Operator {
            operator_type: op, 
            arg1: None,
            arg2: None,
        });

        next_id = if field.next_id == field.id { None } else { Some(field.next_id) };
    }
    else if arity == 1 {
        new_formula = Formula::Operator(Operator {
            operator_type: op, 
            arg1: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: field.id, // Copy previous index 
                next_id: field.next_id, 
                prev_id: field.prev_id 
            }))),
            arg2: None,
        });

        next_id = Some(field.id);
    }
    else {
        new_formula = Formula::Operator(Operator {
            operator_type: op, 
            arg1: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: *next_index, 
                next_id: *next_index + 1, 
                prev_id: if field.prev_id == field.id { *next_index + 1 } else { field.prev_id } 
            }))),
            arg2: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: *next_index + 1, 
                next_id: if field.next_id == field.id { *next_index } else { field.next_id } , 
                prev_id: *next_index
            }))),
        });

        next_id = Some(*next_index);
        *next_index += 2;
    }

    *field_formula = new_formula;

    if arity == 0 {
        if field.prev_id != field.id { 
            formula_as_field(search_field_id_in_proof(proof, Some(field.prev_id)).unwrap()).next_id = field.next_id;
            formula_as_field(search_field_id_in_proof(proof, Some(field.next_id)).unwrap()).prev_id = field.prev_id;
        };
    }
    else if arity == 2 {
        if field.prev_id != field.id { 
            formula_as_field(search_field_id_in_proof(proof, Some(field.prev_id)).unwrap()).next_id = *next_index - 2;
            formula_as_field(search_field_id_in_proof(proof, Some(field.next_id)).unwrap()).prev_id = *next_index - 1;
        };
    }

    return next_id;
}


pub fn formula_as_field(f: &mut Formula) -> &mut FormulaField {
    match f {
        Formula::NotCompleted(formula_field) => formula_field,
        _ => panic!("Expected uncompleted field!"),
    }
}


/// If index is None, returns first field found
pub fn search_field_id_in_proof<'a>(p: &'a mut Proof, index: Option<u32>) -> Option<&'a mut Formula> {
    match search_field_id_in_sequent(&mut p.root, index) {
        Some(res) => Some(res),
        None => {
            for b in p.branches.iter_mut() {
                let res = search_field_id_in_proof(b, index);
                if res.is_some() { return res; }
            }

            return None;
        },
    }
} 

/// If index is None, returns first field found
pub fn search_field_id_in_sequent<'a>(s: &'a mut Sequent, index: Option<u32>) -> Option<&'a mut Formula> {
    for f in s.before.iter_mut() {
        let res = search_field_id_in_formula(f, index);
        if res.is_some() { return res; }
    }

    for f in s.after.iter_mut() {
        let res = search_field_id_in_formula(f, index);
        if res.is_some() { return res; }
    }

    return None;
} 

/// If index is None, returns first field found
pub fn search_field_id_in_formula<'a>(f: &'a mut Formula, index: Option<u32>) -> Option<&'a mut Formula> {
    match f {
        Formula::Operator(operator) => {
            if operator.arg1.is_some() {
                let res = search_field_id_in_formula(operator.arg1.as_mut().unwrap(), index);
                if res.is_some() {
                    return res;
                }
            }

            if operator.arg2.is_some() {
                let res = search_field_id_in_formula(operator.arg2.as_mut().unwrap(), index);
                if res.is_some() {
                    return res;
                }
            }

        return  None;
        },
        Formula::Variable(_) => None,
        Formula::NotCompleted(field) => {
        if index.is_none() || field.id == index.unwrap() {
                return Some(f);
            }
            else {
                return None;
            }
        },
    }
} 

pub fn get_first_unfinished_proof<'a>(p: &'a mut Proof) -> Option<&'a mut Proof> {
    match p.rule_id {
        None => Some(p),
        Some(_) => {
            for b in p.branches.iter_mut() {
                match get_first_unfinished_proof(b) {
                    Some(res) => return Some(res),
                    None => (),
                }
            }

            return None;
        }
    }
}

pub fn sequent_as_empty_proof(s: Sequent) -> Proof {
    return Proof {
        root: s,
        branches: vec![],
        rule_id: None,
    };
}
