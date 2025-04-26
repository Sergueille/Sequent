use super::*;


pub fn get_system() -> super::LogicSystem {
    return super::LogicSystem {
        operators: vec![
            OperatorType::Not, 
            OperatorType::Impl, 
            OperatorType::And, 
            OperatorType::Or, 
            OperatorType::Top, 
            OperatorType::Bottom
        ],
        rules: vec! {
            Box::new(NotI {}),
            Box::new(NotE {}),
            Box::new(ImplI {}),
            Box::new(ImplE {}),
            Box::new(AndI {}),
            Box::new(AndE {}),
            Box::new(Axiom {}),
        },
    }
}


pub struct ImplI { }

impl super::Rule for ImplI {
    fn create_branches(&self, root: &super::Sequent) -> (Option<Vec<super::Sequent>>, u32) {
        return super::execute_on_first_operator_of_type(&root.after, super::OperatorType::Impl, &|i, arg1, arg2| {
            let mut new_seq = root.clone();
            new_seq.after.remove(i);
            new_seq.after.insert(i, arg2.as_ref().unwrap().as_ref().clone());
            new_seq.before.insert(0, arg1.as_ref().unwrap().as_ref().clone());
            
            return (Some(vec![
                new_seq, 
            ]), 0);
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &super::Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "→i"
    }
}


pub struct ImplE { }

impl super::Rule for ImplE {
    fn create_branches(&self, root: &super::Sequent) -> (Option<Vec<super::Sequent>>, u32) {
        if root.after.len() == 0 {
            return (None, 0);
        }

        let mut l = root.clone();
        let mut r = root.clone();

        let impl_right = l.after.remove(0);
        r.after.remove(0);

        l.after.push(Formula::Operator(Operator { 
            operator_type: OperatorType::Impl, 
            arg1: Some(Box::new(Formula::NotCompleted(super::FormulaField {
                id: 0,
                next_id: 0,
                prev_id: 0,
            }))), 
            arg2: Some(Box::new(impl_right)),
        }));

        r.after.push(Formula::NotCompleted(super::FormulaField {
            id: 0,
            next_id: 0,
            prev_id: 0,
        }));

        return (Some(vec![
            l, r, 
        ]), 1);
    }

    fn check_validity(&self, _proof: &super::Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "→e"
    }
}


pub struct AndE {

}

impl super::Rule for AndE {
    fn create_branches(&self, root: &super::Sequent) -> (Option<Vec<super::Sequent>>, u32) {
        if root.after.len() == 0 {
            return (None, 0);
        }
        
        let mut new_seq = root.clone();

        new_seq.after[0] = super::Formula::Operator(super::Operator {
            operator_type: super::OperatorType::And,
            arg1: Some(Box::new(new_seq.after[0].clone())),
            arg2: Some(Box::new(super::Formula::NotCompleted(super::FormulaField {
                id: 0,
                next_id: 0,
                prev_id: 0,
            }))),
        });

        return (Some(vec![
            new_seq, 
        ]), 1);
    }

    fn check_validity(&self, _proof: &super::Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "∧e"
    }
}


pub struct AndI {

}

impl super::Rule for AndI {
    fn create_branches(&self, root: &super::Sequent) -> (Option<Vec<super::Sequent>>, u32) {
        return super::execute_on_first_operator_of_type(&root.after, super::OperatorType::And, &|i, arg1, arg2| {
            let mut seq_1 = root.clone();
            let mut seq_2 = root.clone();

            seq_1.after.remove(i);
            seq_2.after.remove(i);

            seq_1.after.insert(i, arg1.as_ref().unwrap().as_ref().clone());
            seq_2.after.insert(i, arg2.as_ref().unwrap().as_ref().clone());
            
            return (Some(vec![
                seq_1, seq_2, 
            ]), 0);
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &super::Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "∧i"
    }
}

pub struct NotI { }

impl super::Rule for NotI {
    fn create_branches(&self, root: &super::Sequent) -> (Option<Vec<super::Sequent>>, u32) {
        return super::execute_on_first_operator_of_type(&root.after, super::OperatorType::Not, &|i, arg1, _arg2| {
            let mut seq = root.clone();

            seq.after.remove(i);
            seq.after.insert(i, super::Formula::Operator({
                super::Operator {
                    operator_type: super::OperatorType::Bottom,
                    arg1: None,
                    arg2: None,
                }
            }));
            seq.before.push(arg1.as_ref().unwrap().as_ref().clone());

            (Some(vec![seq]), 0)
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &super::Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "¬i"
    }
}


pub struct NotE { }

impl super::Rule for NotE {
    fn create_branches(&self, root: &super::Sequent) -> (Option<Vec<super::Sequent>>, u32) {
        return super::execute_on_first_operator_of_type(&root.after, super::OperatorType::Bottom, &|i, _arg1, _arg2| {
            let mut seq_1 = root.clone();
            let mut seq_2 = root.clone();

            seq_1.after.remove(i);
            seq_2.after.remove(i);

            seq_1.after.insert(i, super::Formula::NotCompleted({
                super::FormulaField {
                    id: 0,
                    next_id: 0,
                    prev_id: 0,
                }
            }));

            seq_2.after.insert(i, super::Formula::Operator(super::Operator { 
                operator_type: super::OperatorType::Not, 
                arg1: Some(Box::new(super::Formula::NotCompleted(super::FormulaField { 
                    id: 0,
                    next_id: 0,
                    prev_id: 0,
                }))), 
                arg2: None
            }));

            (Some(vec![seq_1, seq_2]), 1)
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &super::Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "¬e"
    }
}


pub struct Axiom { }


impl super::Rule for Axiom {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        if root.after.len() != 0 && root.before.contains(&root.after[0]) {
            return (Some(vec![]), 0);
        }
        else {
            return (None, 0);
        }
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "Ax"
    }
}


// TODOlist :) Dont forget to add them in get_system()!
pub struct OrI { }
pub struct OrE { }
pub struct TopI { }
pub struct BottomE { }
pub struct RAA { }
