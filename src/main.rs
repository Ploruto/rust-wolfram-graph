mod point;
use std::collections::HashMap;
mod graph;
mod rule;
use rand::Rng;
use rule::rule::{
    Pattern, Rule,
    Variable::{self, *},
};

fn main() {
    let pattern = Pattern::new(vec![
        vec![Variable::a, Variable::b],
        vec![Variable::a, Variable::c],
        vec![Variable::b, Variable::a],
        vec![Variable::q, Variable::a]
    ]);
    println!("{:?}", pattern.get_distinct_variables_in_pattern());
    println!("{:?}", pattern.get_variable_requirements());
}
