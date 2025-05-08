use super::*;


pub fn get_system() -> LogicSystem {
    return LogicSystem {
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
            Box::new(OrI {}),
            Box::new(OrE {}),
            Box::new(TopI {}),
            Box::new(BottomE {}),
            Box::new(RAA {}),
            Box::new(Axiom {}),
        },
        special_rules: vec! {
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        },
    }
}


pub struct ImplI { }

impl Rule for ImplI {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        return execute_on_first_operator_of_type(&root.after, OperatorType::Impl, &|i, arg1, arg2| {
            let mut new_seq = root.clone();
            new_seq.after.remove(i);
            new_seq.after.insert(i, arg2.as_ref().unwrap().as_ref().clone());
            new_seq.before.insert(0, arg1.as_ref().unwrap().as_ref().clone());
            
            return (Some(vec![
                new_seq, 
            ]), 0);
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "→i"
    }
}


pub struct ImplE { }

impl Rule for ImplE {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        if root.after.len() == 0 {
            return (None, 0);
        }

        let mut l = root.clone();
        let mut r = root.clone();

        let impl_right = l.after.remove(0);
        r.after.remove(0);

        l.after.push(Formula::Operator(Operator { 
            operator_type: OperatorType::Impl, 
            arg1: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: 0,
                next_id: 0,
                prev_id: 0,
            }))), 
            arg2: Some(Box::new(impl_right)),
        }));

        r.after.push(Formula::NotCompleted(FormulaField {
            id: 0,
            next_id: 0,
            prev_id: 0,
        }));

        return (Some(vec![
            l, r, 
        ]), 1);
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "→e"
    }
}

pub struct AndE { }

impl Rule for AndE {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        if root.after.len() == 0 {
            return (None, 0);
        }
        
        let mut new_seq = root.clone();

        new_seq.after[0] = Formula::Operator(Operator {
            operator_type: OperatorType::And,
            arg1: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: 0,
                next_id: 1,
                prev_id: 1,
            }))),
            arg2: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: 1,
                next_id: 0,
                prev_id: 0,
            }))),
        });

        return (Some(vec![
            new_seq, 
        ]), 2);
    }

    fn check_validity(&self, proof: &Proof) -> bool {
        if proof.branches.len() != 1 { return false; }

        let Formula::Operator(op) = &proof.branches[0].root.after[0] else { return false; };

        if op.operator_type != OperatorType::And { return false; }

        return proof.root.after[0] == *op.arg1.as_ref().unwrap().as_ref()
            || proof.root.after[0] == *op.arg2.as_ref().unwrap().as_ref();
    }

    fn display_text(&self) -> &'static str {
        "∧e"
    }
}


pub struct OrI { }

impl Rule for OrI {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        return execute_on_first_operator_of_type(&root.after, OperatorType::Or, &|i, _, _| {
            let mut s = root.clone();

            s.after.remove(i);
            s.after.insert(i, Formula::NotCompleted(FormulaField {
                id: 0,
                next_id: 0,
                prev_id: 0,
            }));
            
            return (Some(vec![s]), 1);
        }, (None, 0));
    }

    fn check_validity(&self, proof: &Proof) -> bool {
        if proof.branches.len() != 1 { return false; }

        let Formula::Operator(op) = &proof.root.after[0] else { unreachable!(); };

        if op.operator_type != OperatorType::Or { unreachable!(); }

        return *op.arg1.as_ref().unwrap().as_ref() == proof.branches[0].root.after[0]
            || *op.arg2.as_ref().unwrap().as_ref() == proof.branches[0].root.after[0];
    }

    fn display_text(&self) -> &'static str {
        "∨i"
    }
}

/* Structs for the rules with left and right version (OrI, AndE)

pub struct AndEL { }

impl Rule for AndEL {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        if root.after.len() == 0 {
            return (None, 0);
        }
        
        let mut new_seq = root.clone();

        new_seq.after[0] = Formula::Operator(Operator {
            operator_type: OperatorType::And,
            arg1: Some(Box::new(new_seq.after[0].clone())),
            arg2: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: 0,
                next_id: 0,
                prev_id: 0,
            }))),
        });

        return (Some(vec![
            new_seq, 
        ]), 1);
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        false
    }

    fn display_text(&self) -> &'static str {
        "∧el"
    }
}

pub struct AndER { }

impl Rule for AndER {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        if root.after.len() == 0 {
            return (None, 0);
        }
        
        let mut new_seq = root.clone();

        new_seq.after[0] = Formula::Operator(Operator {
            operator_type: OperatorType::And,
            arg1: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: 0,
                next_id: 0,
                prev_id: 0,
            }))),
            arg2: Some(Box::new(new_seq.after[0].clone())),
        });

        return (Some(vec![
            new_seq, 
        ]), 1);
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "∧er"
    }
}


pub struct OrIL { }

impl Rule for OrIL {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        return execute_on_first_operator_of_type(&root.after, OperatorType::Or, &|i, arg1, _| {
            let mut s = root.clone();

            s.after.remove(i);
            s.after.insert(i, arg1.as_ref().unwrap().as_ref().clone());
            
            return (Some(vec![s]), 0);
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "∨il"
    }
}

pub struct OrIR { }

impl Rule for OrIR {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        return execute_on_first_operator_of_type(&root.after, OperatorType::Or, &|i, _, arg2| {
            let mut s = root.clone();

            s.after.remove(i);
            s.after.insert(i, arg2.as_ref().unwrap().as_ref().clone());
            
            return (Some(vec![s]), 0);
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "∨ir"
    }
}

*/


pub struct AndI {

}

impl Rule for AndI {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        return execute_on_first_operator_of_type(&root.after, OperatorType::And, &|i, arg1, arg2| {
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

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "∧i"
    }
}

pub struct NotI { }

impl Rule for NotI {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        return execute_on_first_operator_of_type(&root.after, OperatorType::Not, &|i, arg1, _arg2| {
            let mut seq = root.clone();

            seq.after.remove(i);
            seq.after.insert(i, Formula::Operator({
                Operator {
                    operator_type: OperatorType::Bottom,
                    arg1: None,
                    arg2: None,
                }
            }));
            seq.before.push(arg1.as_ref().unwrap().as_ref().clone());

            (Some(vec![seq]), 0)
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "¬i"
    }
}


pub struct NotE { }

impl Rule for NotE {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        return execute_on_first_operator_of_type(&root.after, OperatorType::Bottom, &|i, _arg1, _arg2| {
            let mut seq_1 = root.clone();
            let mut seq_2 = root.clone();

            seq_1.after.remove(i);
            seq_2.after.remove(i);

            seq_1.after.insert(i, Formula::NotCompleted({
                FormulaField {
                    id: 0,
                    next_id: 0,
                    prev_id: 0,
                }
            }));

            seq_2.after.insert(i, Formula::Operator(Operator { 
                operator_type: OperatorType::Not, 
                arg1: Some(Box::new(Formula::NotCompleted(FormulaField { 
                    id: 0,
                    next_id: 0,
                    prev_id: 0,
                }))), 
                arg2: None
            }));

            (Some(vec![seq_1, seq_2]), 1)
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "¬e"
    }
}


pub struct Axiom { }


impl Rule for Axiom {
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

pub struct OrE { }


impl Rule for OrE {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        let mut a = root.clone();
        let mut b = a.clone();
        let mut c = a.clone();

        a.before.push(Formula::NotCompleted(FormulaField { 
            id: 0,
            next_id: 1,
            prev_id: 1,
        }));

        b.before.push(Formula::NotCompleted(FormulaField { 
            id: 1,
            next_id: 0,
            prev_id: 0,
        }));

        c.after.remove(0);
        
        c.after.push(Formula::Operator(Operator { 
            operator_type: OperatorType::Or, 
            arg1: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: 0,
                next_id: 1,
                prev_id: 1,
            }))), 
            arg2: Some(Box::new(Formula::NotCompleted(FormulaField {
                id: 1,
                next_id: 0,
                prev_id: 0,
            }))), 
        }));

        return (Some(vec![a, b, c]), 2)
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "∨e"
    }
}


pub struct TopI { }

impl Rule for TopI {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        return execute_on_first_operator_of_type(&root.after, OperatorType::Top, &|_, _, _| {
            return (Some(vec![]), 0);
        }, (None, 0));
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "⊤i"
    }
}


pub struct BottomE { }

impl Rule for BottomE {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        let mut s = root.clone();

        s.after.remove(0);
        s.after.push(Formula::Operator(Operator { operator_type: OperatorType::Bottom, arg1: None, arg2: None }));
        
        return (Some(vec![s]), 0);
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "⊥e"
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct RAA { }

impl Rule for RAA {
    fn create_branches(&self, root: &Sequent) -> (Option<Vec<Sequent>>, u32) {
        let mut s = root.clone();

        let after = s.after.remove(0);
        s.before.push(Formula::Operator(Operator { operator_type: OperatorType::Not, arg1: Some(Box::new(after)), arg2: None }));
        s.after.push(Formula::Operator(Operator { operator_type: OperatorType::Bottom, arg1: None, arg2: None }));
        
        return (Some(vec![s]), 0);
    }

    fn check_validity(&self, _proof: &Proof) -> bool {
        true
    }

    fn display_text(&self) -> &'static str {
        "RAA"
    }
}

