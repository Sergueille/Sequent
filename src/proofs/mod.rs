
#![allow(dead_code)]

mod rendering;

type Variable = u32;

/// Each rule will be a dedicated type that implement this.
pub trait Rule {
    /// Create proof template from the sequent. Returns None if not compatible.
    fn create_branches(&self, root: &Sequent) -> Option<Vec<Proof>>; 
    /// Check if the sequents above the root of the proofs corresponds to the rule.
    fn check_validity(&self, proof: &Proof) -> bool; 
}

/// A proof tree.
pub struct Proof {
    root: Sequent,
    branches: Vec<Proof>,
    rule: Box<dyn Rule>,
}

/// A sequent!
/// 
/// I used vec for both sides, will be useful if we want to implement other logic systems.
pub struct Sequent {
    before: Vec<Formula>,
    after: Vec<Formula>,

    cached_text_section: Option<notan::glyph::Section<'static>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorType {
    Not, Impl, And, Or, Top, Bottom,
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
    NotCompleted(u32),
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

/// Create operator with NotCompleted
fn create_uncompleted_operator(op: OperatorType, next_index: &mut u32) -> Formula {
    let arity = get_operator_arity(op);

    let res = Formula::Operator(Operator {
        operator_type: op,
        arg1: if arity >= 1 { Some(Box::new(Formula::NotCompleted(*next_index))) } else { None },
        arg2: if arity >= 2 { Some(Box::new(Formula::NotCompleted(*next_index + 1))) } else { None },
    });

    *next_index += arity;

    return res;
}

