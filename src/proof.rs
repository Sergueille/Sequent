
#![allow(dead_code)]

type Variable = u32;

trait Rule {
    fn create_branches<'a>(&self, root: &'a Sequent<'a>) -> Vec<Proof<'a>>; 
    fn check_validity<'a>(&self, proof: &Proof<'a>) -> bool; 
}

struct Proof<'a> {
    root: Sequent<'a>,
    branches: Vec<Proof<'a>>,
    rule: &'a dyn Rule,
}

struct Sequent<'a> {
    before: Vec<Formula<'a>>,
    after: Formula<'a>,
}

enum OperatorType {
    Not, Impl, And, Or, Top, Bottom, // ...
}

struct Operator<'a> {
    pub operator_type: OperatorType,
    pub arg1: Option<&'a Formula<'a>>,
    pub arg2: Option<&'a Formula<'a>>,
    pub arity: u8
}

enum Formula<'a> {
    Operator(Operator<'a>),
    Variable(Variable),
    NotCompleted,
}

