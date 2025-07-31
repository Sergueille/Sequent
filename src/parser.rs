use crate::Sequent;
use crate::Formula;
use crate::Operator;
use crate::OperatorType;

use crate::HashMap;

use std::str::Chars;
use std::fs;
// use std::fs::File;
// use std::io::Read;

#[derive(Clone)]
pub enum Difficulty {
    Immediate,
    Easy,
    Medium,
    Hard,
}

#[derive(Clone)]
pub struct Level{
    pub id: usize,
    pub name: String,
    pub seq: Sequent,
    pub difficulty: Difficulty,
    pub raa: bool
}

impl Level {
    pub fn empty() -> Level{
        let seq = Sequent {before: vec![], after: vec![]};
        Level {id: 0, name: "".to_string(), seq, difficulty: Difficulty::Immediate, raa: false}
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
            Formula::Operator(Operator {operator_type: OperatorType::Impl, arg1: Some(Box::new(left)), arg2: Some(Box::new(right))})
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
        Some('_') => {
            Formula::Operator(Operator {operator_type: OperatorType::Bottom, arg1: None, arg2: None})
        },
        Some('°') => {
            Formula::Operator(Operator {operator_type: OperatorType::Top, arg1: None, arg2: None})
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

    // println!("seq: {}", seq);
    let mut vars: HashMap<char, u32> = HashMap::new();
    let mut i = 0;

    let s: Vec<&str> = seq.split('-').collect();
    // println!("{}", s.len());
    assert!(s.len() == 2);
    let before_s: Vec<&str> = s[0].split(',').collect();
    let after_s: Vec<&str> = s[1].split(',').collect();

    // println!("before_s: {:?}", before_s);
    let mut before = Vec::with_capacity(before_s.len());
    let mut after = Vec::with_capacity(after_s.len());

    for j in 0..before_s.len(){
        if before_s[j] != "" {
            before.push(var_r(&mut before_s[j].chars(), ligne_number, &mut vars, &mut i));
        }
    }

    for j in 0..after_s.len(){
        after.push(var_r(&mut after_s[j].chars(), ligne_number, &mut vars, &mut i));
    }


    return Sequent {before, after}
}

pub fn parse_ligne(ligne: &str, ligne_number: usize) -> Level{

    let infos: Vec<&str> = ligne.split(';').collect();

    // println!("n : {}", ligne_number);
    // println!("ligne : {}", ligne);
    assert!(infos.len() == 6);

    let mut result = Level::empty();

    match usize::from_str_radix(&infos[0].replace(" ",""),10){
        Ok(u) => result.id = u,
        Err(_) => println!("Syntax error on ligne {}: invalid seq id", ligne_number),
    }

    result.name = infos[1].trim().replace("\"", "").to_string();

    result.seq = parse_sequent(&infos[2].replace(" ",""), ligne_number);
    
    match usize::from_str_radix(infos[3].trim(),10){
        Ok(u) => {
            let reffs = [ Difficulty::Immediate, Difficulty::Easy, Difficulty::Medium, Difficulty::Hard ];
            result.difficulty = reffs[u].clone();
        }
        Err(_) => println!("Syntax error on ligne {}: invalide difficult", ligne_number),
    }

    // println!("infos[2] : {}", infos[2]);
    match usize::from_str_radix(infos[4].trim(),10){
        Ok(u) => result.raa = u == 1,
        Err(_) => println!("Syntax error on ligne {}: invalide raa", ligne_number),
    }

    result
}

pub fn parse_file(path: &str) -> Vec<Level>{

    let contents = fs::read_to_string(path.to_string()).expect("Should have been able to read the file");
    let lignes: Vec<&str> = contents.split('\n').collect();

    let mut levels = Vec::with_capacity(lignes.len());

    for i in 0..lignes.len(){
        if lignes[i].get(0..1) != Some("#") && lignes[i] != "" {
            levels.push(parse_ligne(lignes[i], i+1));
        }
    }

    levels
}

impl ToString for Difficulty
{
    fn to_string(&self) -> String {
        String::from(match self {
            Difficulty::Immediate => "Immediate",
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        })
    }
}

// */

