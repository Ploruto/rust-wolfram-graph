mod point;
use std::collections::HashMap;
mod graph;
mod rule;
use rand::Rng;
use rule::rule::{Rule, Variable};

fn main() {
    // create a graph with 2000 points and 4000 random relations between the existing points
    let mut graph = graph::graph::Graph::new();
    for _ in 0..2000 {
        graph.create_point();
    }
    let mut rng = rand::thread_rng();

    let all_points = graph.get_points();
    let mut points1 = Vec::new();
    let mut points2 = Vec::new();
    for _ in 0..2000 {
        let index1 = rng.gen_range(0..all_points.len());
        let index2 = rng.gen_range(0..all_points.len());
        let point1 = all_points.keys().nth(index1).unwrap().clone();
        let point2 = all_points.keys().nth(index2).unwrap().clone();
        points1.push(point1);
        points2.push(point2)
    }
    graph.add_relations(points1, points2);

    let find_pattern: Vec<Vec<Variable>> = vec![vec![Variable::a, Variable::b], vec![Variable::a, Variable::c], vec![Variable::a, Variable::d], vec![Variable::a, Variable::q]];
    let r = graph.get_possible_variable_resolutions(find_pattern);

}
