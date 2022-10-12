use std::collections::HashMap;

use crate::point;

use super::super::point::point::Point;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Variable {
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
}

pub struct Rule {
    /* the variables inside these pattern are placeholders for
    arbitrary point, where the variable corresponds to the point
    and when a != b then the point (a) is not equal to the point (b)
    */
    find_pattern: Vec<Vec<Variable>>, // for ex.: [[a, b, w], [a, c]]
    replace_pattern: Vec<Vec<Variable>>, // for ex.: [[a, x, q], [b, x], [c, x], [b, c]]
}





impl Rule {
    pub fn new(find_pattern: Vec<Vec<Variable>>, replace_pattern: Vec<Vec<Variable>>) -> Self {
        Self {
            find_pattern,
            replace_pattern,
        }
    }

    pub fn get_distinct_variables_in_pattern(rule_pattern: Vec<Vec<Variable>>) -> Vec<Variable> {
        let mut different_variables = Vec::new();
        for pattern in rule_pattern {
            for variable in pattern {
                if !different_variables.contains(&variable) {
                    different_variables.push(variable);
                }
            }
        }
        different_variables
    }

    // if a variable is not in the find pattern, but is in the replace pattern, then it is a new variable
    pub fn find_variables_to_create(find_pattern: &Vec<Vec<Variable>>, replace_pattern: Vec<Vec<Variable>>) -> Vec<Variable> {
        let mut variables_to_create = Vec::new();
        let distinct_variables_in_find_pattern = Self::get_distinct_variables_in_pattern(find_pattern.clone());
        let distinct_variables_in_replace_pattern = Self::get_distinct_variables_in_pattern(replace_pattern.clone());
        // find the difference between the two vectors
        for variable in distinct_variables_in_replace_pattern {
            if !distinct_variables_in_find_pattern.contains(&variable) {
                variables_to_create.push(variable);
            }
        }
        variables_to_create
    }
    /* How to find all the `requirements` for every 'point' in the find_pattern:
    1. Find all the different variables in the find_pattern .: vars
    2. For every variable in the vars, find all the points that come right after it in the find_pattern
     */
    pub fn get_find_pattern_requirements(find_pattern: &Vec<Vec<Variable>>) -> HashMap<Variable, Vec<Variable>> {
        let mut requirements = HashMap::new();

        // find the different variables in the find_pattern
        let mut different_variables = Vec::new();
        for relation in find_pattern {
            for variable in relation {
                if !different_variables.contains(&variable) {
                    different_variables.push(variable);
                }
            }
        }

        // find the requirements for every variable
        for variable in different_variables {
            let mut requirements_for_variable = Vec::new();
            for relation in find_pattern {
                if relation.contains(&variable) {
                    let index = relation.iter().position(|x| x == variable).unwrap();
                    if index + 1 < relation.len() {
                        requirements_for_variable.push(relation[index + 1].clone());
                    }
                }
            }
            if !requirements_for_variable.is_empty() {
                requirements.insert(variable.clone(), requirements_for_variable);
            }
        }
        requirements
    } 

    }

/// A pattern is a list of relations, where each relation is a list of variables
/// ex.: [[a, b, w], [a, c], [b, a]] -> [a,b,w] means that a -> b -> w
pub struct Pattern {
    pattern: Vec<Vec<Variable>>,
}

impl Pattern {
    pub fn new(pattern: Vec<Vec<Variable>>) -> Self {
        Self { pattern }
    }

    pub fn get_pattern(&self) -> Vec<Vec<Variable>> {
        self.pattern.clone()
    }

    /// Returns a vector of all the different variables in the pattern
    /// for ex.: [[a, b, w], [a, c]] -> [a, b, w, c]
    pub fn get_distinct_variables_in_pattern(&self) -> Vec<&Variable> {
        let mut different_variables = Vec::new();
        for pattern in &self.pattern {
            for variable in pattern {
                if !different_variables.contains(&variable) {
                    different_variables.push(&variable);
                }
            }
        }
        different_variables
    }

    /// Returns a HashMap with the requirements for every variable in the pattern
    /// ex.: { a: [a,b], b: [d,c], c: [a] }
    pub fn get_variable_requirements(&self) -> HashMap<Variable, Vec<Variable>> {
        let mut requirements = HashMap::new();

        // find the different variables in the find_pattern
        let different_variables = self.get_distinct_variables_in_pattern();

        // find the requirements for every variable
        for variable in different_variables {
            let mut requirements_for_variable = Vec::new();
            for relation in &self.pattern {
                if relation.contains(&variable) {
                    let index = relation.iter().position(|x| x == variable).unwrap();
                    if index + 1 < relation.len() {
                        requirements_for_variable.push(relation[index + 1].clone());
                    }
                }
            }
            if !requirements_for_variable.is_empty() {
                requirements.insert(variable.clone(), requirements_for_variable);
            }
        }
        requirements
    }
    
}





#[test]
fn test_rule() {
    let find_pattern =
    vec![vec![Variable::a, Variable::b],
    vec![Variable::a, Variable::c],
    vec![Variable::b, Variable::c]];

    let mut expected = HashMap::new();
    expected.insert(Variable::a, vec![Variable::b, Variable::c]);
    expected.insert(Variable::b, vec![Variable::c]);

    let requirements = Rule::get_find_pattern_requirements(&find_pattern);

    assert_eq!(requirements, expected);
}