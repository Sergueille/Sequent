use crate::Sequent;
use crate::Formula;
use crate::Operator;
use crate::OperatorType;

use crate::HashMap;

use std::str::Chars;

pub struct Level{
    id: usize,
    name: String,
    seq: Sequent,
    diff: bool,
    raa: bool
}

impl Level {
    pub fn empty() -> Level{
        let seq = Sequent {before: vec![], after: vec![]};
        Level {id: 0, name: "".to_string(), seq, diff: true, raa: false}
    }
}

// /*

fn var_r(buff: &mut Chars<'_>, ligne_number: usize, vars: &mut HashMap<char, u32>, i: &mut u32) -> Formula{

    let left = var_p(buff, ligne_number, vars, i);

    match buff.next() {
        Some ('&') => {
            let right = var_p(buff, ligne_number, vars, i);
            Formula::Operator(Operator {operator_type: OperatorType::And, arg1: Some(Box::new(left)), arg2: Some(Box::new(right))})
        },
        Some ('|') => {
            let right = var_p(buff, ligne_number, vars, i);
            Formula::Operator(Operator {operator_type: OperatorType::Or, arg1: Some(Box::new(left)), arg2: Some(Box::new(right))})
        },
        Some ('>') => {
            let right = var_p(buff, ligne_number, vars, i);
            Formula::Operator(Operator {operator_type: OperatorType::Or, arg1: Some(Box::new(left)), arg2: Some(Box::new(right))})
        }
        Some (u) => panic!("Syntax error on ligne {}: unexpected token {}", ligne_number, u),
        None => left,
    }
}

fn var_p(buff: &mut Chars<'_>, ligne_number: usize, vars: &mut HashMap<char, u32>, i: &mut u32) -> Formula{

    match buff.next() {
        Some ('(') => {
            let r = var_r(buff, ligne_number, vars, i);
            match buff.next() {
                Some (')') => r,
                Some(u) => panic!("Syntax error on ligne {}: expected ')', found '{}'", ligne_number, u),
                None => panic!("Syntax error on ligne {}: expected ')', found nothing", ligne_number)
            }
        },
        Some('!') => {
            let p = var_p(buff, ligne_number, vars,i);
            Formula::Operator(Operator {operator_type: OperatorType::Not, arg1: Some(Box::new(p)), arg2: None})
        },
        Some(u) => {
            if !vars.contains_key(&u) {
                vars.insert(u,*i);
                *i += 1;
            }

            Formula::Variable(*(vars.get(&u).unwrap()))
        },
        None => panic!("Syntax error on ligne {}: expected formula, found nothing", ligne_number),
    }
}

pub fn parse_sequent(seq: &str, ligne_number: usize) -> Sequent{

    let mut vars: HashMap<char, u32> = HashMap::new();
    let mut i = 0;

    let s: Vec<&str> = seq.split(' ').collect();
    assert!(s.len() == 2);
    let before_s: Vec<&str> = s[0].split(',').collect();
    let after_s: Vec<&str> = s[1].split(',').collect();

    let mut before = Vec::with_capacity(before_s.len());
    let mut after = Vec::with_capacity(after_s.len());

    for j in 0..before_s.len(){
        before.push(var_r(&mut before_s[j].chars(), ligne_number, &mut vars, &mut i));
    }

    for j in 0..after_s.len(){
        after.push(var_r(&mut before_s[j].chars(), ligne_number, &mut vars, &mut i));
    }


    return Sequent {before: vec![], after: vec![]}
}

pub fn parse_ligne(ligne: &str, ligne_number: usize) -> Level{

    let infos: Vec<&str> = ligne.split(';').collect();

    assert!(infos.len() == 5);

    let mut result = Level::empty();

    match usize::from_str_radix(&infos[0].replace(" ",""),1){
        Ok(u) => result.id = u,
        Err(_) => println!("Syntax error on ligne {}: invalid seq id", ligne_number),
    }

    result.name = infos[1].trim().to_string();

    result.seq = parse_sequent(&infos[2].replace(" ",""), ligne_number);
    
    match usize::from_str_radix(infos[3],1){
        Ok(u) => result.diff = u == 1,
        Err(_) => println!("Syntax error on ligne {}: invalide difficult", ligne_number),
    }

    match usize::from_str_radix(infos[4],1){
        Ok(u) => result.diff = u == 1,
        Err(_) => println!("Syntax error on ligne {}: invalide raa", ligne_number),
    }

    result
}

// */

