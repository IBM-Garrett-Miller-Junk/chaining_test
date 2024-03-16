

const EMPTY_STRING: String = String::new();
#[derive(Clone)]
pub struct HornClause {
    pub antecedents: Vec<String>,
    pub consequent: String,
}

struct HornSolver {
    facts: Vec<String>,
    rules: Vec<HornClause>,
    num_facts : usize,
    num_rules : usize,
}

impl HornSolver {
    pub fn add_fact(solver: &mut Self, fact:  String) {
        solver.facts.push(fact);
        solver.num_facts+=1;
    }

    pub fn add_rule(solver: &mut Self,
                    clause: HornClause) {
        solver.rules.push(clause);
        solver.num_rules+=1;
    }

    pub fn is_fact_true(solver: &mut Self, fact: String) -> bool {
        for i in 0..solver.facts.len() {
            if solver.facts[i] == fact {
                return true;
            }
        }
        false
    }

    pub fn apply_rule(solver: &mut Self, rule: HornClause) -> bool {
        for antecedent in rule.antecedents.into_iter() {
            if!HornSolver::is_fact_true(solver, antecedent.clone()){
                return false;
            }
        }
        solver.facts.push(rule.consequent.clone());
        solver.num_facts+=1;
        true
    }

    pub fn forward_chaining(solver: &mut Self) {
        let mut new_facts: bool = true;
        while new_facts{
            new_facts = false;
            for i in 0..solver.num_rules {
                if !HornSolver::is_fact_true(solver, solver.rules[i].consequent.clone()){
                    new_facts |= HornSolver::apply_rule(solver, solver.rules[i].clone());
                }
            }
        }
    }

    pub fn backward_chaining(solver: &Self) {
    }

    pub fn query(solver: &mut Self, query: String) -> bool {
        HornSolver::forward_chaining(solver);
        HornSolver::is_fact_true(solver, query.clone())
    }
}

fn main() {
    let mut solver = HornSolver{
        facts:Vec::new(),
        rules: Vec::new(),
        num_facts:0,
        num_rules:0
    };
    HornSolver::add_fact(&mut solver, "A".to_string());
    HornSolver::add_fact(&mut solver, "B".to_string());
    let mut clause = HornClause{
        antecedents: Vec::new(),
        consequent: EMPTY_STRING
    };
    clause.antecedents.push("A".to_string());
    clause.antecedents.push("B".to_string());
    clause.consequent = "C".to_string();
    HornSolver::add_rule(&mut solver, clause);
    let query = HornSolver::query(&mut solver, "C".to_string());
    if query {
        println!("Success");
    } else {
        println!("Failed");
    }
}