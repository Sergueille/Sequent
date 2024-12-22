
pub fn get_system() -> super::LogicSystem {
    return super::LogicSystem {
        operators: vec![
            super::OperatorType::Not, 
            super::OperatorType::Impl, 
            super::OperatorType::And, 
            super::OperatorType::Or, 
            super::OperatorType::Top, 
            super::OperatorType::Bottom
        ],
        rules: vec! {
            Box::new(ImplI {}),
            Box::new(ImplE {}),
        },
    }
}


pub struct ImplI {

}


impl super::Rule for ImplI {
    fn create_branches(&self, root: &super::Sequent, _next_field_id: &mut u32) -> Option<Vec<super::Proof>> {
        for (i, formula) in root.after.iter().enumerate() {
            match formula {
                super::Formula::Operator(op) => {
                    if op.operator_type == super::OperatorType::Impl {
                        let mut new_seq = root.clone();
                        new_seq.after.remove(i);
                        new_seq.after.insert(i, op.arg2.as_ref().unwrap().as_ref().clone());
                        new_seq.before.insert(0, op.arg1.as_ref().unwrap().as_ref().clone());
                        
                        return Some(vec![
                            super::sequent_as_empty_proof(new_seq), 
                        ]);
                    }
                    else {
                        return None;
                    }
                },
                _ => ()
            }
        }

        return None;
    }

    fn check_validity(&self, proof: &super::Proof) -> bool {
        todo!()
    }

    fn display_text(&self) -> &'static str {
        todo!()
    }
}


pub struct ImplE {

}

impl super::Rule for ImplE {
    fn create_branches(&self, root: &super::Sequent, _next_field_id: &mut u32) -> Option<Vec<super::Proof>> {
        if root.before.len() == 0 || root.after.len() == 0 {
            return None;
        }

        let mut new_seq = root.clone();

        new_seq.after.insert(0, super::Formula::Operator(super::Operator {
            operator_type: super::OperatorType::Impl,
            arg1: Some(Box::new(new_seq.before[0].clone())),
            arg2: Some(Box::new(new_seq.after[0].clone())),
        }));

        new_seq.before.remove(0);
        new_seq.after.remove(1);

        return Some(vec![
            super::sequent_as_empty_proof(new_seq), 
        ])
    }

    fn check_validity(&self, proof: &super::Proof) -> bool {
        todo!()
    }

    fn display_text(&self) -> &'static str {
        todo!()
    }
}


