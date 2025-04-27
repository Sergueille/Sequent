// Reimplementation of the Tp 6 on Sequent calculation
use crate::*;

// A little change in the struct to help with the proofs
#[derive(Clone)]
pub struct SequentCalc {
    pub before: Vec<Formula>,
    pub before_var: Vec<Formula>,
    pub after: Vec<Formula>,
    pub after_var: Vec<Formula>,
}

// Transform a sequent to the new format
fn sequent_to_sequentcalc(sequent: Sequent) -> SequentCalc{
    SequentCalc {
        before: sequent.before,
        before_var: vec![],
        after: sequent.after,
        after_var: vec![],
    }
}


// Verify if a formula is contain in a vector of formulas
fn member(formula: &Formula, list: &Vec<Formula>) -> bool{
    for elmt in list.iter() {if elmt == formula {return true}}
    false
}

// Check if the bottom rule can be applied
fn bottom_calc(seq: &SequentCalc) -> bool{
    let bottom = Formula::Operator(Operator {operator_type: OperatorType::Bottom, arg1:  None, arg2: None}); 
    member(&bottom, &seq.before) || member(&bottom, &seq.before_var)
}


// Check if the top rule can be applied
fn top_calc(seq: &SequentCalc) -> bool{
    let top = Formula::Operator(Operator {operator_type: OperatorType::Top, arg1:  None, arg2: None}); 
    member(&top, &seq.after) || member(&top, &seq.after_var)
}

// Check if the axiom rule can be applied
fn axiom_calc(seq: &SequentCalc) -> bool{
    for elmt in seq.before.iter(){ if member(&elmt, &seq.after) || member(&elmt, &seq.after_var) {return true}}
    for elmt in seq.before_var.iter(){ if member(&elmt, &seq.after) || member(&elmt, &seq.after_var) {return true}}
    false
}


// Advance the calculation by one step, by mutating the queue.
// return Some bool (wether or not the original sequent was true) if the proof is finished
// return None otherwise
fn step_one_calc(queue: &mut Vec<SequentCalc>) -> Option<bool>{
    let mut seq;
    match queue.pop() {
        Some(elmt) => seq = elmt,
        None => return Some(true)   // All branches have been proven !
    }
    if bottom_calc(&seq) || top_calc(&seq) || axiom_calc(&seq) {return None} // End of this branch
    
    match seq.before.pop() {
        Some(elmt) => {
            let formula = elmt;
            match &formula {
                Formula::Operator(operator) => {
                    match operator.operator_type {
                        OperatorType::Not => {
                            match &operator.arg1 {
                                Some(child) => {
                                    seq.after.push(*child.clone());
                                    queue.push(seq);
                                },
                                None => unreachable!() // Error handeling I dont know how to do that :(
                            };
                            None
                        },
                        OperatorType::Impl => {
                            match &operator.arg1 {
                                Some(child) => {
                                    let mut seq_bis = seq.clone();
                                    seq_bis.after.push(*child.clone());
                                    queue.push(seq_bis);
                                },
                                None => {unreachable!()} // Error handeling I dont know how to do that :(
                            };
                            match &operator.arg2 {
                                Some(child) => {
                                    seq.before.push(*child.clone());
                                    queue.push(seq);
                                },
                                None => unreachable!() // Error handeling I dont know how to do that :(
                            }
                            None

                        },
                        OperatorType::And => {
                            match &operator.arg1 {
                                Some(child) => {
                                    seq.before.push(*child.clone());
                                },
                                None => {unreachable!()} // Error handeling I dont know how to do that :(
                            };
                            match &operator.arg2 {
                                Some(child) => {
                                    seq.before.push(*child.clone());
                                },
                                None => unreachable!() // Error handeling I dont know how to do that :(
                            };
                            queue.push(seq);
                            None
                        },
                        OperatorType::Or => {
                            match &operator.arg1 {
                                Some(child) => {
                                    let mut seq_bis = seq.clone();
                                    seq_bis.before.push(*child.clone());
                                    queue.push(seq_bis);
                                },
                                None => {unreachable!()} // Error handeling I dont know how to do that :(
                            };
                            match &operator.arg2 {
                                Some(child) => {
                                    seq.before.push(*child.clone());
                                    queue.push(seq);
                                },
                                None => unreachable!() // Error handeling I dont know how to do that :(
                            }
                            None
                        },
                        OperatorType::Top | OperatorType::Bottom => {
                            seq.before_var.push(formula);
                            queue.push(seq);
                            None
                        }
                    }
                },
                Formula::Variable(_variable) => {
                    seq.before_var.push(formula);
                    queue.push(seq);
                    None
                }
                Formula::NotCompleted(_int) => {
                    None
                }
            }
        },
        None => {
            match seq.after.pop() {
                Some(elmt) => {
                    let formula = elmt;
                    match &formula {
                        Formula::Operator(operator) => {
                            match operator.operator_type {
                                OperatorType::Not => {
                                    match &operator.arg1 {
                                        Some(child) => {
                                            seq.before.push(*child.clone());
                                            queue.push(seq);
                                        },
                                        None => unreachable!() // Error handeling I dont know how to do that :(
                                    };
                                    None
                                },
                                OperatorType::Impl => {
                                    match &operator.arg1 {
                                        Some(child) => {
                                            seq.before.push(*child.clone());
                                        },
                                        None => {unreachable!()} // Error handeling I dont know how to do that :(
                                    };
                                    match &operator.arg2 {
                                        Some(child) => {
                                            seq.after.push(*child.clone());
                                        },
                                        None => unreachable!() // Error handeling I dont know how to do that :(
                                    }
                                    queue.push(seq);
                                    None

                                },
                                OperatorType::And => {
                                    match &operator.arg1 {
                                        Some(child) => {
                                            let mut seq_bis = seq.clone();
                                            seq_bis.after.push(*child.clone());
                                            queue.push(seq_bis);
                                        },
                                        None => {unreachable!()} // Error handeling I dont know how to do that :(
                                    };
                                    match &operator.arg2 {
                                        Some(child) => {
                                            seq.after.push(*child.clone());
                                        },
                                        None => unreachable!() // Error handeling I dont know how to do that :(
                                    };
                                    None
                                },
                                OperatorType::Or => {
                                    match &operator.arg1 {
                                        Some(child) => {
                                            seq.after.push(*child.clone());
                                        },
                                        None => {unreachable!()} // Error handeling I dont know how to do that :(
                                    };
                                    match &operator.arg2 {
                                        Some(child) => {
                                            seq.after.push(*child.clone());
                                        },
                                        None => unreachable!() // Error handeling I dont know how to do that :(
                                    }
                                    queue.push(seq);
                                    None
                                },
                                OperatorType::Top | OperatorType::Bottom => {
                                    seq.after_var.push(formula);
                                    queue.push(seq);
                                    None
                                }
                            }
                        },
                        Formula::Variable(_variable) => {
                            seq.after_var.push(formula);
                            queue.push(seq);
                            None
                        }
                        Formula::NotCompleted(_int) => {
                            None
                        }
                    }
                },
                None => Some(false), // The branch isn't provable
            }
        }
    }
}

fn sequent_calculation(seq: SequentCalc) -> bool{
    let mut queue = vec![];
    queue.push(seq);
    loop {
        match step_one_calc(&mut queue) {
            Some(res) => return res,
            None => continue,
        }
    }
}

pub fn proof_or_fake(seq: Sequent) -> bool{
    sequent_calculation(sequent_to_sequentcalc(seq))
}
