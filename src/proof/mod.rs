
pub mod rendering;
pub mod calcul;
pub mod natural_logic;

type Variable = u32;

pub const MAX_VARIABLE_COUNT: u32 = 10;

/// Each rule will be a dedicated type that implement this.
pub trait Rule {
    /// Create proof template from the sequent. Returns None if not compatible. Also returns the number of created empty fields
    /// next_field_id should be increased if new empty fields are created. Otherwise, it must not be modified.
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32); 
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
    
    pub last_focused_time: f32,
    pub creation_time: f32,
    pub rule_set_time: f32,
}

/// A sequent!
/// 
/// I used vec for both sides, will be useful if we want to implement other logic systems.
#[derive(Clone)]
pub struct Sequent {
    pub before: Vec<Formula>,
    pub after: Vec<Formula>,
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
    pub special_rules: Vec<Option<Box<dyn Rule>>>,
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

pub fn get_operator_symbol(op: OperatorType) -> &'static str {
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
    let mut fields = search_fields_by_id_in_proof(proof, Some(field_id));
    let first_field = formula_as_field(fields[0]).clone();
    
    for field_formula in fields.into_iter() {
        let new_formula = Formula::Variable(var);
        
        *field_formula = new_formula;
    };

    if first_field.next_id == field_id {
        return None;
    }
    else {
        for f in search_fields_by_id_in_proof(proof, Some(first_field.prev_id)).into_iter(){
            formula_as_field(f).next_id = first_field.next_id;
        };
        for f in search_fields_by_id_in_proof(proof, Some(first_field.next_id)).into_iter() {
            formula_as_field(f).prev_id = first_field.prev_id;
        };

        return Some(first_field.next_id);
    }
}

/// Create an operator with NotCompleted as arguments, then places it in field with field_id. Returns the id of the next field to be focused, if there is any left. 
pub fn place_uncompleted_operator(op: OperatorType, field_id: u32, proof: &mut Proof, next_index: &mut u32) -> Option<u32> {
    let arity = get_operator_arity(op);
    let mut fields = search_fields_by_id_in_proof(proof, Some(field_id));
    let first_field = formula_as_field(fields[0]).clone();

    for field_formula in fields.into_iter() {
        let field = formula_as_field(field_formula).clone();

        let new_formula;

        if arity == 0 {
            new_formula = Formula::Operator(Operator {
                operator_type: op, 
                arg1: None,
                arg2: None,
            });
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
        }

        *field_formula = new_formula;
    };

    let next_id;
    if arity == 0 {
        if first_field.prev_id != first_field.id { 
            for f in search_fields_by_id_in_proof(proof, Some(first_field.prev_id)).into_iter() {
                formula_as_field(f).next_id = first_field.next_id;
            };
            for f in search_fields_by_id_in_proof(proof, Some(first_field.next_id)).into_iter() {
                formula_as_field(f).prev_id = first_field.prev_id;
            };
        };

        next_id = if first_field.next_id == first_field.id { None } else { Some(first_field.next_id) };
    }
    else if arity == 1 {
        next_id = Some(first_field.id);
    }
    else {
        if first_field.prev_id != first_field.id { 
            for f in search_fields_by_id_in_proof(proof, Some(first_field.prev_id)).into_iter() {
                formula_as_field(f).next_id = *next_index;
            };
            for f in search_fields_by_id_in_proof(proof, Some(first_field.next_id)).into_iter() {
                formula_as_field(f).prev_id = *next_index + 1;
            };
        };

        next_id = Some(*next_index);
        *next_index += 2;
    }

    return next_id;
}


pub fn formula_as_field(f: &mut Formula) -> &mut FormulaField {
    match f {
        Formula::NotCompleted(formula_field) => formula_field,
        _ => panic!("Expected uncompleted field!"),
    }
}


/// If index is None, returns all fields
pub fn search_fields_by_id_in_proof(p: &mut Proof, index: Option<u32>) -> Vec<&mut Formula> {
    let mut res = Vec::new();
    _search_fields_by_id_in_proof(p, index,&mut res);
    return res;
} 

fn _search_fields_by_id_in_proof<'a>(p: &'a mut Proof, index: Option<u32>, res: &mut Vec<&'a mut Formula>) {
    search_field_id_in_sequent(&mut p.root, index, res);

    for b in p.branches.iter_mut() {
        _search_fields_by_id_in_proof(b, index, res);
    }
} 


/// If index is None, returns all fields
fn search_field_id_in_sequent<'a>(s: &'a mut Sequent, index: Option<u32>, res: &mut Vec<&'a mut Formula>) {
    for f in s.before.iter_mut() {
        search_field_id_in_formula(f, index, res);
    }

    for f in s.after.iter_mut() {
        search_field_id_in_formula(f, index, res);
    }
} 

/// If index is None, returns all fields
fn search_field_id_in_formula<'a>(f: &'a mut Formula, index: Option<u32>, res: &mut Vec<&'a mut Formula>) {
    match f {
        Formula::Operator(operator) => {
            if operator.arg1.is_some() {
                search_field_id_in_formula(operator.arg1.as_mut().unwrap(), index, res);
            }

            if operator.arg2.is_some() {
                search_field_id_in_formula(operator.arg2.as_mut().unwrap(), index, res);
            }
        },
        Formula::Variable(_) => (),
        Formula::NotCompleted(field) => {
            if index.is_none() || field.id == index.unwrap() {
                res.push(f);
            }
        },
    }
} 

pub fn get_first_unfinished_proof(p: &mut Proof) -> Option<&mut Proof> {
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

pub fn sequent_as_empty_proof(s: Sequent, time: f32) -> Proof {
    return Proof {
        root: s,
        branches: vec![],
        rule_id: None,
        last_focused_time: f32::NEG_INFINITY,
        creation_time: time,
        rule_set_time: f32::NEG_INFINITY,
    };
}

pub fn execute_on_first_operator_of_type<T>(formulas: &[Formula], op_type: OperatorType, f: &dyn Fn(usize, &Option<Box<Formula>>, &Option<Box<Formula>>) -> T, otherwise: T) -> T {
    for (i, formula) in formulas.iter().enumerate() {
        match formula {
            super::Formula::Operator(op) => {
                if op.operator_type == op_type {
                    return f(i, &op.arg1, &op.arg2);
                }
            },
            _ => ()
        }
    }

    return otherwise;
}
